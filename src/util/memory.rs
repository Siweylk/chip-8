
pub struct Memory{
    pub mem:[u8;4096],
}

impl Memory{
    pub fn new() -> Self{
        Self{
            mem: [0;4096],
        }
    }

    pub fn read(&self, addr:usize) -> u8{
        self.mem[addr]
    }

    pub fn write(&mut self, addr:usize, dat:u8){
        self.mem[addr] = dat;
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

    pub fn push(&mut self, val:usize){
        self.stack.push(val);
    }

    pub fn pop(&mut self)-> usize{
        self.stack.pop().expect("The stack is empty")
    }

    //Tal vez añadir peek, empty, size....

}