use super::memory::{Memory, Stack};
use super::screen::{Screen};
use super::timers::{DelayTimer, SoundTimer};

use minifb::Key;
use rand::Rng;

pub struct Machine{
    mem: Memory,
    stack: Stack,
    displ: Screen,
    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
    pc: u16,
    i: u16,
    reg: [u8; 16],
}

impl Machine{
    pub fn new() -> Self{
        Self{
            mem: Memory::new(),
            stack: Stack::new(),
            displ: Screen::new(),
            delay_timer: DelayTimer::new(),
            sound_timer: SoundTimer::new(),
            pc: 0x200,
            i: 0,
            reg: [0; 16],
        }
    }

    

    pub fn load_char_sprites(&mut self, chars: Vec<u8>){
        self.mem.write_char_sprites(chars);
    }

    pub fn fetch(&mut self)-> u16{     
        let instruction : u16 = ((self.mem.read(self.pc) as u16) << 8) | (self.mem.read(self.pc+1) as u16);
        self.pc += 2;
        instruction
    }

    pub fn instr_00e0(&mut self){
        self.displ.clear_screen();
    }

    pub fn instr_1nnn(&mut self, dir: u16){
        self.pc = dir;
    }

    pub fn instr_2nnn(&mut self, dir: u16){
        self.stack.push(self.pc);
        self.pc = dir;
    }

    pub fn instr_00ee(&mut self){
        self.pc = self.stack.pop();
    }

    pub fn instr_3xnn(&mut self, reg: u8, val: u8){
        if self.reg[reg as usize] == val{
            self.pc += 2;
        }
    }

    pub fn instr_4xnn(&mut self, reg: u8, val: u8){
        if self.reg[reg as usize] != val{
            self.pc += 2;
        }
    }

    pub fn instr_5xy0(&mut self, regx: u8, regy: u8){
        if self.reg[regx as usize] == self.reg[regy as usize]{
            print!("Ha entrado en el if");
            self.pc += 2;
        }
    }

    pub fn instr_9xy0(&mut self, regx: u8, regy: u8){
        if self.reg[regx as usize] != self.reg[regy as usize]{
            print!("Ha entrado en el if");
            self.pc += 2;
        }
    }

    pub fn instr_6xnn(&mut self, reg: u8, val: u8){
        self.reg[reg as usize] = val;
    }

    pub fn instr_7xnn(&mut self, reg: u8, val: u8){
        self.reg[reg as usize] += val;
    }

    pub fn instr_8xy0(&mut self, regx: u8, regy: u8){
        self.reg[regx as usize] = self.reg[regy as usize];
    }

    pub fn instr_8xy1(&mut self, regx: u8, regy: u8){
        self.reg[regx as usize] |= self.reg[regy as usize];
    }  

    pub fn instr_8xy2(&mut self, regx: u8, regy: u8){
        self.reg[regx as usize] &= self.reg[regy as usize];
    } 

    pub fn instr_8xy3(&mut self, regx: u8, regy: u8){
        self.reg[regx as usize] ^= self.reg[regy as usize];
    }   

    pub fn instr_8xy4(&mut self, regx: u8, regy: u8){
        if (self.reg[regx as usize]as usize + self.reg[regy as usize] as usize) > 255{
            self.reg[regx as usize] = 0xFF;

            //Set carry flag to 1
            self.reg[0x0F] = 1;
        }else{
            self.reg[regx as usize] += self.reg[regy as usize];
            //Set carry flag to 0
            self.reg[0x0F] = 0;
        }
    } 

    pub fn instr_8xy5(&mut self, regx: u8, regy: u8){
        if self.reg[regx as usize] > self.reg[regy as usize]{
            self.reg[regx as usize] -= self.reg[regy as usize];

            //Set carry flag to 1
            self.reg[0x0F] = 1;
        }else{
            self.reg[regx as usize] = 0;

            //Set carry flag to 0
            self.reg[0x0F] = 0;
        }
    } 

    pub fn instr_8xy6(&mut self, reg: u8){
        self.reg[0x0F] = self.reg[reg as usize] & 1;
        self.reg[reg as usize] = self.reg[reg as usize] >> 1;
    }

    pub fn instr_8xy7(&mut self, regx: u8, regy: u8){
        if self.reg[regy as usize] > self.reg[regx as usize]{
            self.reg[regx as usize] = self.reg[regy as usize] -self.reg[regx as usize];

            //Set carry flag to 1
            self.reg[0x0F] = 1;
        }else{
            self.reg[regx as usize] = 0;

            //Set carry flag to 0
            self.reg[0x0F] = 0;
        }
    } 

    pub fn instr_8xye(&mut self, reg: u8){
        self.reg[0xF] = self.reg[reg as usize] >> 7;
        self.reg[reg as usize] = self.reg[reg as usize] << 1;
    }

    pub fn instr_annn(&mut self, val: u16){
        self.i = val;
    }

    pub fn instr_bnnn(&mut self, val: u16){
        self.pc= val + self.reg[0x00] as u16;
    }

    pub fn instr_cxnn(&mut self, reg:u8, val:u8){
        let mut rng = rand::thread_rng();
        let rand: u8 = rng.r#gen();
        print!("Random number: {}", rand);

        self.reg[reg as usize] = val & rand;
    }

    pub fn instr_dxyn(&mut self, regx:u8, regy:u8, val:u16){
        
        let x_coord = self.reg[regx as usize] % 64;
        let mut y_coord = self.reg[regy as usize] % 32;
        self.reg[0x0F] = 0;


        //SE ESTAN DIBUJANDO LOS BYTES AL REVES 11110000-->00001111
        //Start drawing
        for i in 0..val{
            let byte = self.mem.read(self.i+i);
            for j in 0..8{
                if (byte >> 7-j) & 1 == 1{
                    self.displ.flip_pixel((x_coord+j) as usize, y_coord as usize);
                    //print!("Se ha dibujado en: ({},{})\n", x_coord+j,y_coord);

                    if !self.displ.get_pixel((x_coord+j) as usize, y_coord as usize){
                        self.reg[0x0F]=1;
                    }
                }

                /*else{
                    print!("No se ha dibujado en: ({},{}), val: {}\n", x_coord+j,y_coord, (byte >> j) & 1);
                }*/
            }
            y_coord += 1;
        }
        self.displ.update();
    }



    pub fn instr_ex9e(&mut self, reg: u8){
        
        let register_val: u8 = self.reg[reg as usize];

        match register_val{
            0x1=> if self.displ.window.is_key_down(Key::Key1) {self.pc += 2},
            0x2=> if self.displ.window.is_key_down(Key::Key2) {self.pc += 2},
            0x3=> if self.displ.window.is_key_down(Key::Key3) {self.pc += 2},
            0xC=> if self.displ.window.is_key_down(Key::Key4) {self.pc += 2},
            0x4=> if self.displ.window.is_key_down(Key::Q) {self.pc += 2},
            0x5=> if self.displ.window.is_key_down(Key::W) {self.pc += 2},
            0x6=> if self.displ.window.is_key_down(Key::E) {self.pc += 2},
            0xD=> if self.displ.window.is_key_down(Key::R) {self.pc += 2},
            0x7=> if self.displ.window.is_key_down(Key::A) {self.pc += 2},
            0x8=> if self.displ.window.is_key_down(Key::S) {self.pc += 2},
            0x9=> if self.displ.window.is_key_down(Key::D) {self.pc += 2},
            0xE=> if self.displ.window.is_key_down(Key::F) {self.pc += 2},
            0xA=> if self.displ.window.is_key_down(Key::Z) {self.pc += 2},
            0x0=> if self.displ.window.is_key_down(Key::X) {self.pc += 2},
            0xB=> if self.displ.window.is_key_down(Key::C) {self.pc += 2},
            0xF=> if self.displ.window.is_key_down(Key::V) {self.pc += 2},

            _=> print!("Error: Unknown value in register {}", reg),
        }
    }

    pub fn instr_exa1(&mut self, reg: u8){
        
        let register_val: u8 = self.reg[reg as usize];

        match register_val{
            0x1=> if !self.displ.window.is_key_down(Key::Key1) {self.pc += 2},
            0x2=> if !self.displ.window.is_key_down(Key::Key2) {self.pc += 2},
            0x3=> if !self.displ.window.is_key_down(Key::Key3) {self.pc += 2},
            0xC=> if !self.displ.window.is_key_down(Key::Key4) {self.pc += 2},
            0x4=> if !self.displ.window.is_key_down(Key::Q) {self.pc += 2},
            0x5=> if !self.displ.window.is_key_down(Key::W) {self.pc += 2},
            0x6=> if !self.displ.window.is_key_down(Key::E) {self.pc += 2},
            0xD=> if !self.displ.window.is_key_down(Key::R) {self.pc += 2},
            0x7=> if !self.displ.window.is_key_down(Key::A) {self.pc += 2},
            0x8=> if !self.displ.window.is_key_down(Key::S) {self.pc += 2},
            0x9=> if !self.displ.window.is_key_down(Key::D) {self.pc += 2},
            0xE=> if !self.displ.window.is_key_down(Key::F) {self.pc += 2},
            0xA=> if !self.displ.window.is_key_down(Key::Z) {self.pc += 2},
            0x0=> if !self.displ.window.is_key_down(Key::X) {self.pc += 2},
            0xB=> if !self.displ.window.is_key_down(Key::C) {self.pc += 2},
            0xF=> if !self.displ.window.is_key_down(Key::V) {self.pc += 2},

            _=> print!("Error: Unknown value in register {}", reg),
        }
    }

    pub fn instr_fx07(&mut self, reg: u8){
        self.reg[reg as usize] = self.delay_timer.get_delay_timer();
    }

    pub fn instr_fx15(&mut self, reg: u8){
        self.delay_timer.set_delay_timer(self.reg[reg as usize]);
    }

    pub fn instr_fx18(&mut self, reg: u8){
        self.sound_timer.set_sound_timer(self.reg[reg as usize]);
    }

    pub fn instr_fx1e(&mut self, reg:u8){
        self.i += self.reg[reg as usize] as u16;
    }

    pub fn instr_fx0a(&mut self, reg: u8){
        if self.displ.window.is_key_down(Key::Key1) {self.reg[reg as usize]=0x01}
        else if self.displ.window.is_key_down(Key::Key2) {self.reg[reg as usize]=0x02}
        else if self.displ.window.is_key_down(Key::Key3) {self.reg[reg as usize]=0x03}
        else if self.displ.window.is_key_down(Key::Key4) {self.reg[reg as usize]=0x0C}
        else if self.displ.window.is_key_down(Key::Q) {self.reg[reg as usize]=0x04}
        else if self.displ.window.is_key_down(Key::W) {self.reg[reg as usize]=0x05}
        else if self.displ.window.is_key_down(Key::E) {self.reg[reg as usize]=0x06}
        else if self.displ.window.is_key_down(Key::R) {self.reg[reg as usize]=0x0D}
        else if self.displ.window.is_key_down(Key::A) {self.reg[reg as usize]=0x07}
        else if self.displ.window.is_key_down(Key::S) {self.reg[reg as usize]=0x08}
        else if self.displ.window.is_key_down(Key::D) {self.reg[reg as usize]=0x09}
        else if self.displ.window.is_key_down(Key::F) {self.reg[reg as usize]=0x0E}
        else if self.displ.window.is_key_down(Key::Z) {self.reg[reg as usize]=0x0A}
        else if self.displ.window.is_key_down(Key::X) {self.reg[reg as usize]=0x00}
        else if self.displ.window.is_key_down(Key::C) {self.reg[reg as usize]=0x0B}
        else if self.displ.window.is_key_down(Key::V) {self.reg[reg as usize]=0x0F}
        else{self.pc -= 2};
    }
    
    pub fn instr_fx29(&mut self, reg: u8){

        self.i = 0x50 + self.reg[reg as usize] as u16 * 5;

    }

    pub fn instr_fx33(&mut self, reg: u8){
        
        let hundreds:u8 = self.reg[reg as usize] / 100;
        let decimals:u8 = (self.reg[reg as usize] % 100) / 10;
        let units:u8 = self.reg[reg as usize] % 10;
        
        self.mem.write(self.i, hundreds);
        self.mem.write(self.i+1, decimals);
        self.mem.write(self.i+2, units);
    }

    pub fn instr_fx55(&mut self, reg: u8){
        for i in  0..reg{
            self.mem.write(self.i+i as u16, self.reg[i as usize]);
        }
    }

    pub fn instr_fx65(&mut self, reg: u8){
        for i in  0..reg{
            self.reg[i as usize] = self.mem.read(self.i+i as u16);
        }
    }

}