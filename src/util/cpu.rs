use super::machine::{Machine};
use std::sync::{Arc, atomic::{AtomicU8}};





pub fn run(cpu_tick_counter: Arc<AtomicU8>){
    
    let mut machine: Machine = Machine::new();

    let mut opcode: u16;

    //Fetch first instruction when the machine starts
    opcode = machine.fetch();

    loop{
        
        //Execute and fetch ADD CLOCK SYNCRONIZATION!!!!!!!!!!!!!!!!
        decode_call_instruction(&mut machine, opcode);
        opcode = machine.fetch();
    }
}

pub fn decode_call_instruction(machine: &mut Machine, opcode: u16){


    let mut v: usize;

    match opcode{

        //Clear Screen
        0x00E0 => machine.instr_00e0(),

        //Jump to dir
        v if (v & 0xF000) == 0x1000 =>{
            let dir: usize = v as usize & 0x0FFF;
            machine.instr_1nnn(dir);
        }

        //Call subrutine pushing current pc
        v if (v & 0xF000) == 0x2000 =>{
            let dir: usize = v as usize & 0x0FFF;
            machine.instr_2nnn(dir);
        }

        //End subrutine popping previous pc
        0x00EE =>machine.instr_00ee(),

        //Conditional jump if reg==val
        v if (v & 0xF000) == 0x3000 =>{
            let reg: usize = v as usize & 0x0F00;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_3xnn(reg, val);
        }    

        //Conditional jump if reg!=val
        v if (v & 0xF000) == 0x4000 =>{
            let reg: usize = v as usize & 0x0F00;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_3xnn(reg, val);
        }   

        //Conditional jump if regX==regY
        v if (v & 0xF000) == 0x5000 =>{
            let reg1: usize = v as usize & 0x0F00;
            let reg2: usize = v as usize & 0x00F0;
            machine.instr_5xy0(reg1, reg2);
        }  

        //Conditional jump if regX!=regY
        v if (v & 0xF000) == 0x9000 =>{
            let reg1: usize = v as usize & 0x0F00;
            let reg2: usize = v as usize & 0x00F0;
            machine.instr_9xy0(reg1, reg2);
        } 

        //Set register x to value
        v if (v & 0xF000) == 0x6000=>{
            let reg: usize = v as usize & 0x0F00;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_6xnn(reg, val);
        }  



        _ => print!("Unknown opcode"),
    }
}


