use super::machine::{Machine};
use std::sync::{Arc, atomic::{AtomicU8}};





pub fn run(cpu_tick_counter: Arc<AtomicU8>){
    
    let mut machine: Machine = Machine::new();

    let mut opcode: u16;

    //Fetch first instruction when the machine starts
    opcode = machine.fetch();

    loop{
        
        //Execute and fetch ADD CLOCK SYNCRONIZATION!!!!!!!!!!!!!!!!
        decode_callInstruction(opcode);
        opcode = machine.fetch();
    }
}

pub fn decode_call_nstruction(opcode: u16){

    match opcode{
        1 => print!("Sigo mañana"),
        _ => print!("AAAAAAAAAAA"),
    }
}


