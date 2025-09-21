use super::machine::{Machine};
use std::sync::{Arc, atomic::{AtomicU8}};


use crate::assets::sprites::font::CHARS;





pub fn run(cpu_tick_counter: Arc<AtomicU8>){
    
    let  mut machine: Machine = Machine::new();

    //Load character sprites in memmory
    machine.load_char_sprites(CHARS.to_vec());

    let mut opcode: u16;


    /*decode_call_instruction(&mut machine, 0x6000);
    decode_call_instruction(&mut machine, 0x6100);
    decode_call_instruction(&mut machine, 0x620A);
    decode_call_instruction(&mut machine, 0xF229);

    decode_call_instruction(&mut machine, 0xD015);*/


    //Fetch first instruction when the machine starts
    /*opcode = machine.fetch();

    loop{
        
        //Execute and fetch ADD CLOCK SYNCRONIZATION!!!!!!!!!!!!!!!!
        decode_call_instruction(&mut machine, opcode);
        opcode = machine.fetch();
    }*/
}

pub fn decode_call_instruction(machine: &mut Machine, opcode: u16){

    //machine.keep_screen_alive();


    match opcode{

        //0x00E0: Clear Screen
        0x00E0 => machine.instr_00e0(),

        //0x1NNN: Jump to NNN
        v if (v & 0xF000) == 0x1000 =>{
            let dir: u16 = v as u16 & 0x0FFF;
            machine.instr_1nnn(dir);
        }

        //0x2NNN: Call subrutine in NNN pushing current pc
        v if (v & 0xF000) == 0x2000 =>{
            let dir: u16 = v as u16 & 0x0FFF;
            machine.instr_2nnn(dir);
        }

        //0x00EE: End subrutine popping previous pc
        0x00EE =>machine.instr_00ee(),


        //0x3XNN: Conditional jump if VX equals NN
        v if (v & 0xF000) == 0x3000 =>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_3xnn(reg, val);
        }    

        //0x4XNN: Conditional jump if VX not equal NN
        v if (v & 0xF000) == 0x4000 =>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_4xnn(reg, val);
        }   

        //0x5XY0: Conditional jump if VX equals VY
        v if (v & 0xF000) == 0x5000 =>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_5xy0(regx, regy);
        }  

        //0x5XY0: Conditional jump if VX not equal VY
        v if (v & 0xF000) == 0x9000 =>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_9xy0(regx, regy);
        } 

        //0x6XNN: Set VX to value NN
        v if (v & 0xF000) == 0x6000=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_6xnn(reg, val);
        } 

        //0x7XNN: Add NN value to VX
        v if (v & 0xF000) == 0x7000=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            let val: u8 = v as u8 & 0x00FF;
            machine.instr_7xnn(reg, val);
        }

        //0x8XY0: Set VX to the value of VY
        v if (v & 0xF00F) == 0x8000=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy0(regx, regy);
        }  

        //0x8XY1: Set VX to the logicl OR between VX and VY
        v if (v & 0xF00F) == 0x8001=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy1(regx, regy);
        }  

        //0x8XY2: Set VX to the logicl AND between VX and VY
        v if (v & 0xF00F) == 0x8002=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy2(regx, regy);
        } 

        //0x8XY3: Set VX to the logicla XOR between VX and VY
        v if (v & 0xF00F) == 0x8003=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy3(regx, regy);
        } 

        //0x8XY4: Set VX to VX + VY, affects the carry flag
        v if (v & 0xF00F) == 0x8004=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy4(regx, regy);
        } 

        //0x8XY5: Set VX to VX - VY, affects the carry flag: if VX > VY --> VF=1 else VF=0
        v if (v & 0xF00F) == 0x8005=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy5(regx, regy);
        }

        //0x8XY6: Shift VX one bit to the right and set VF to the shifted bit
        v if (v & 0xF00F) == 0x8006=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_8xy6(reg);
        }

        //x8XY7: Set VX to VY - VX, affects the carry flag: if VY > VX --> VF=1 else VF=0
        v if (v & 0xF00F) == 0x8007=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            machine.instr_8xy7(regx, regy);
        }  

        //0x8XYE: Shift VX one bit to the left and set VF to the shifted bit
        v if (v & 0xF00F) == 0x800E=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_8xye(reg);
        }

        //0xANNN: Set I to NNN
        v if (v & 0xF000) == 0xA000=>{
            let val: u16 = v as u16 & 0x0FFF;
            machine.instr_annn(val);
        }

        //0xBNNN: Jump to the NNN address plus the value in V0
        v if (v & 0xF000) == 0xB000=>{
            let val: u16 = v as u16 & 0x0FFF;
            machine.instr_bnnn(val);
        }

        //0xCXNN: Write in VX the logical and between NN and a random number
        v if (v & 0xF000) == 0xC000=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            let val: u8 = (v & 0x00FF) as u8;
            machine.instr_cxnn(reg, val);
        }

        //0xDXYN: Draw N pixels tall sprite in (VX, VY)
        v if (v & 0xF000) == 0xD000=>{
            let regx: u8 = ((v >> 8) & 0x0F) as u8;
            let regy: u8 = ((v >> 4) & 0x0F) as u8;
            let val: u16 = v & 0x000F as u16;
            machine.instr_dxyn(regx, regy, val);
        }

        //0xEX9E: Skip one instruction if the key corresponding to VX is pressed
        v if (v & 0xF0FF) == 0xE09E=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_ex9e(reg);
        }

        //0xEXA1: Skip one instruction if the key corresponding to VX is not pressed
        v if (v & 0xF0FF) == 0xE0A1=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_exa1(reg);
        }

        //0xFX07: Sets VX to the value of the delay timer
        v if (v & 0xF0FF) == 0xE007=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx07(reg);
        }

        //0xFX15: Sets the delay timer to the value in VX
        v if (v & 0xF0FF) == 0xF015=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx15(reg);
        }

        //0xFX18: Sets sound timer to the value in VX
        v if (v & 0xF0FF) == 0xF018=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx18(reg);
        }

        //0xFX1E: Add VX to I
        v if (v & 0xF0FF) == 0xF01E=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx1e(reg);
        }

        //0xFX0A: Stops execution until a key is pressed and stores its value in VX
        v if (v & 0xF0FF) == 0xF00A=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx0a(reg);
        }

        //0xFX29: I is set to the character in VX
        v if (v & 0xF0FF) == 0xF029=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx29(reg);
        }

        //0xFX33: Converts the value of VX into three digits and stores them at the I address in memmory
        v if (v & 0xF0FF) == 0xF033=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx33(reg);
        }

        //0xFX55: Stores the values from V0 to VX on memmory starting in address I
        v if (v & 0xF0FF) == 0xF055=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx55(reg);
        }

        //0xFX65: Stores the values from I to I+X of memmory in the registers
        v if (v & 0xF0FF) == 0xF065=>{
            let reg: u8 = ((v >> 8) & 0x0F) as u8;
            machine.instr_fx65(reg);
        }

        _ => print!("Unknown opcode"),
    }
}


