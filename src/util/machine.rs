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

    //Add each instruction as a function
    
}