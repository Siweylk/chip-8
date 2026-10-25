
pub struct Memory{
    pub mem:[u8;4096],
}

impl Memory{
   
    pub fn new() -> Self{
        Self{
            mem: [0;4096],
        }
    }

    pub fn read(&self, addr:u16) -> u8{
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr:u16, dat:u8){
        self.mem[addr as usize] = dat;
    }

    pub fn write_char_sprites(&mut self, chars: Vec<u8>){

        for i in 0..79{
            self.mem[0x50+i] = chars[i];
        }

    }
}

//Creo un struct stack, para mantener la coherencia con el resto de estructuras del programa
pub struct Stack{
    pub stack: Vec<usize>,
}

impl Stack{

    pub fn new() -> Self{
        Self{
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val:u16){
        self.stack.push(val as usize);
    }

    pub fn pop(&mut self)-> u16{
        self.stack.pop().expect("The stack is empty") as u16
    }

    //Tal vez añadir peek, empty, size....

}