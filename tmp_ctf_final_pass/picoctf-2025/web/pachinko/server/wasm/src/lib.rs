use wasm_bindgen::prelude::*;
use verilog_macro::synth_cpu;

#[wasm_bindgen]
pub extern "C" fn process(ptr: &mut [u8]) -> u32 {
    let raw_ptr = ptr.as_mut_ptr();
    
    let nand = |a: usize, b: usize, y: usize| unsafe {
        *raw_ptr.add(y) = !(*raw_ptr.add(a) & *raw_ptr.add(b));
    };

    synth_cpu!("../../verilog/cpu.json", nand);
    
    0
}
