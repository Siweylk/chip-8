use super::memory::{Memory, Stack};
use super::screen::{Screen};
use super::timers::{DelayTimer, SoundTimer};

pub struct Machine{
    mem: Memory,
    stack: Stack,
    displ: Screen,
    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
    pc: usize,
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

    pub fn fetch(&mut self)-> u16{
            
        let instruction : u16 = ((self.mem.read(self.pc) as u16) << 8) | (self.mem.read(self.pc+1) as u16);
        self.pc += 2;
        instruction
    }

    pub fn instr_00e0(&mut self){
        self.displ.clear_screen();
    }

    pub fn instr_1nnn(&mut self, dir: usize){
        self.pc = dir;
    }

    pub fn instr_2nnn(&mut self, dir: usize){
        self.stack.push(self.pc);
        self.pc = dir;
    }

    pub fn instr_00ee(&mut self){
        self.pc = self.stack.pop();
    }

    pub fn instr_3xnn(&mut self, reg: usize, val: u8){
        if self.reg[reg] == val{
            self.pc += 2;
        }
    }

    pub fn instr_4xnn(&mut self, reg: usize, val: u8){
        if self.reg[reg] != val{
            self.pc += 2;
        }
    }

    pub fn instr_5xy0(&mut self, reg1: usize, reg2: usize){
        if self.reg[reg1] == self.reg[reg2]{
            self.pc += 2;
        }
    }

    pub fn instr_9xy0(&mut self, reg1: usize, reg2: usize){
        if self.reg[reg1] != self.reg[reg2]{
            self.pc += 2;
        }
    }

    pub fn instr_6xnn(&mut self, reg:usize, val: u8){
        self.reg[reg] = val;
    }
    
}