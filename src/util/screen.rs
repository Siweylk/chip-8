use minifb::{Window,WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen{
    window: Window,
    buffer: Vec<u32>,
}

impl Screen{
    pub fn new()-> Self{

        let window = Window::new(
            "CHIP 8",
            WIDTH,
            HEIGHT,
            WindowOptions{
                resize: false,
                scale: minifb::Scale::X8,
                ..WindowOptions::default()
            },
        ).expect("Unable to open window");

        let buffer = vec![0x00000000; WIDTH * HEIGHT];

        Self {window, buffer}
    }

    pub fn flip_pixel(&mut self, x: usize, y:usize){
        if x < WIDTH && y < HEIGHT{
            self.buffer[y*WIDTH+x] ^= 0xFFFFFFFF;
            print!("Pixel {:?} {:?}\n", x, y);
        }
    }

    pub fn update(&mut self){
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
        print!("Screen updated\n");
    }

    pub fn clear_screen(&mut self){
        for x in 0..64{
            for y in 0..32{
                self.buffer[y*WIDTH+x] = 0x00000000;
            }
        }
        self.update();
    }
}