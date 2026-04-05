const fs = require('fs');
const path = require('path');

// JSON parsing utilities
function loadCpuSignals() {
    const json = JSON.parse(fs.readFileSync(path.join(__dirname, '../verilog/cpu.json'), 'utf8'));
    return {
        clock: getBitFromJson(json, "clock"),
        addr: getBitsFromJson(json, "addr"),
        inp_val: getBitsFromJson(json, "inp_val"),
        out_val: getBitsFromJson(json, "out_val"),
        reset: getBitFromJson(json, "reset"),
        write_enable: getBitFromJson(json, "write_enable"),
        halted: getBitFromJson(json, "halted"),
        flag: getBitFromJson(json, "flag"),
    };
}

function getBitFromJson(json, name) {
    const bits = getBitsFromJson(json, name);
    if (bits.length !== 1) throw new Error(`Expected single bit for ${name}`);
    return bits[0];
}

function getBitsFromJson(json, name) {
    const ports = json.modules.cpu.netnames;
    const bits = ports[name].bits;
    return bits.map(v => {
        if (typeof v === 'string' && v === '0') return 0;
        const n = parseInt(v);
        if (n === 0 || n === 1) throw new Error(`Unexpected 0 or 1 as non-string value`);
        return n;
    });
}

// Bit manipulation utilities
function getBitsValue(state, bits) {
    let result = 0;
    for (let i = 0; i < bits.length; i++) {
        result |= ((state[bits[i]] >> 7) & 1) << i;
    }
    return result;
}

function setBits(state, bits, value) {
    for (let i = 0; i < bits.length; i++) {
        state[bits[i]] = ((value >> i) & 1) ? 255 : 0;
    }
}

function splitBits(bits, at) {
    return [bits.slice(0, at), bits.slice(at)];
}

// Circuit validation utilities
function checkInt(value) {
    if (value === undefined) return false;
    if (typeof value !== 'number') return false;
    if (value !== Math.floor(value)) return false;
    return value > 0 && value <= 0xFFFF;
}

function serializeCircuit(circuit, program, inputState, outputState) {
    const memory = new Uint8Array(65536); // 64KB memory

    // Copy program at start
    memory.set(program);

    // Serialize output state at 0x1000
const outputView = new Uint16Array(memory.buffer, 0x1000);
    outputView[0] = outputState.length;
    outputView.set(outputState, 1);

    // Serialize input state at 0x2000
    const inputView = new Uint16Array(memory.buffer, 0x2000);
    inputView.set(inputState, outputState.length + 1);

    // Serialize circuit at 0x3000
    const circuitView = new Uint16Array(memory.buffer, 0x3000);
    circuit.forEach((gate, i) => {
        const offset = i * 3;
        circuitView[offset] = gate.input1;
        circuitView[offset + 1] = gate.input2;
        circuitView[offset + 2] = gate.output;
    });

    return memory;
}

module.exports = {
    loadCpuSignals,
    getBitsValue,
    setBits,
    splitBits,
    checkInt,
    serializeCircuit
}; 