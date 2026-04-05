const { 
    loadCpuSignals, 
    getBitsValue, 
    setBits, 
    splitBits,
} = require('./utils');

const { process } = require('./wasm/pkg/verilog_ctf_wasm.js');

function runCPU(memory) {
    const state = new Uint8Array(100_000);
    const signals = loadCpuSignals();

    // Reset sequence
    process(state);
    state[signals.reset] = 255;
    process(state);
    state[signals.reset] = 0;
    process(state);

    let flag = false;
    const MAX_CYCLES = 500000;

    for (let cycle = 0; cycle < MAX_CYCLES; cycle++) {
        // Toggle clock
        state[signals.clock] ^= 255;
        process(state);

        // On clock low edge
        if (state[signals.clock] === 0) {
            // Handle memory writes
            if (state[signals.write_enable] === 255) {
                const addr = getBitsValue(state, signals.addr);
                const val = getBitsValue(state, signals.out_val);
                memory[addr] = val & 0xFF;
                memory[addr + 1] = (val >> 8) & 0xFF;
            }

            // Handle memory reads
            const addr = getBitsValue(state, signals.addr);
            const [first_byte, second_byte] = splitBits(signals.inp_val, 8);
            setBits(state, first_byte, memory[addr]);
            setBits(state, second_byte, memory[addr + 1]);

            // Check halted and flag
            if (state[signals.halted] === 255) {
                break;
            }
            if (state[signals.flag] === 255) {
                flag = true;
            }
        }
    }

    return flag;
}

module.exports = {
    runCPU
}; 