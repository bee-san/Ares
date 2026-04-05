const express = require('express');
const morgan = require('morgan');
const path = require('path');
const fs = require('fs').promises;
const { 
    checkInt,
    serializeCircuit
} = require('./utils');
const { runCPU } = require('./cpu');

const app = express();
const port = process.env.PORT || 3000;

app.use(morgan('dev'));
app.use(express.json());

app.use(express.static(path.join(__dirname, 'public')));

const FLAG1 = process.env.FLAG1 || 'FLAG1';
const FLAG2 = process.env.FLAG2 || 'FLAG2';

function doRun(res, memory) {
  const flag = runCPU(memory);
  const result = memory[0x1000] | (memory[0x1001] << 8);
  if (memory.length < 0x1000) {
    return res.status(500).json({ error: 'Memory length is too short' });
  }

  let resp = "";

  if (flag) {
    resp += FLAG2 + "\n";
  } else {
    if (result === 0x1337) {
      resp += FLAG1 + "\n";
    } else if (result === 0x3333) {
      resp += "wrong answer :(\n";
    } else {
      resp += "unknown error code: " + result;
    }
  }

  res.status(200).json({ status: 'success', flag: resp });
}

// Admin endpoint
app.post('/flag', async (req, res) => {
  if (req.body.flag1 !== FLAG1 || req.body.flag2 !== FLAG2) {
    return res.status(400).json({ error: 'Invalid password' });
  }

  const binary = await fs.readFile('./programs/flag.bin');
  const memory = new Uint8Array(binary.length);
  memory.set(binary);

  doRun(res, memory);
});

// Add the check endpoint
app.post('/check', async (req, res) => {
    const circuit = req.body.circuit;

    if (!Array.isArray(circuit) || 
        !circuit.every(entry => checkInt(entry?.input1) && 
                                checkInt(entry?.input2) && 
                                checkInt(entry?.output))) {
        return res.status(400).end();
    }

    const program = await fs.readFile('./programs/nand_checker.bin');
    
    // Generate random input state with only 0x0000 or 0xffff values
    const inputState = new Uint16Array(4);
    for (let i = 0; i < 4; i++) {
        inputState[i] = Math.random() < 0.5 ? 0x0000 : 0xffff;
    }
    
    // Create output state as inverse of input
    const outputState = new Uint16Array(4);
    for (let i = 0; i < 4; i++) {
        outputState[i] = inputState[i] === 0xffff ? 0x0000 : 0xffff;
    }
    
    const serialized = serializeCircuit(
        circuit,
        program,
        inputState,
        outputState
    );

    doRun(res, serialized);
});

// Start server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
}); 