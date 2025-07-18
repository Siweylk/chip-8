use std::sync::{Arc, atomic::{AtomicU32}};

use super::memory::{Memory, Stack};


pub fn cpu(cpu_tick_counter: Arc<AtomicU32>){
    
    let mut mem: Memory = Memory::new();
    let mut stack: Stack = Stack::new();

    let mut PC: usize = 200;


}

pub fn fetch(pc: &mut usize, mem: Memory) -> u16{

    let instruction : u16 = ((mem.read(*pc) as u16) << 8) | (mem.read(*pc+1) as u16);
    *pc += 2;

    instruction

}

pub fn execute(opcode: u32){

}