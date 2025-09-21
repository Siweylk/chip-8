use std::thread;
use std::sync::{Arc, atomic::{AtomicU32, AtomicU8}};
use std::time::Duration;

use crate::util::cpu::{decode_call_instruction, run};
use crate::util::screen::{Screen};
use crate::util::machine::{Machine};

mod util;
mod assets;
use crate::assets::sprites::font::CHARS;

fn main() {

    let  mut machine: Machine = Machine::new();

    machine.load_char_sprites(CHARS.to_vec());


    //Escribir en registros
    decode_call_instruction(&mut machine, 0x6000);
    decode_call_instruction(&mut machine, 0x6100);
    decode_call_instruction(&mut machine, 0x6207);

    //Set I to the V2 character sprite pos in memory (A)
    decode_call_instruction(&mut machine, 0xF229);

    // Draw A in the V0, V1 position (5 bits tall) (0,0)
    decode_call_instruction(&mut machine, 0xD005);

    //Escribir en registros
    decode_call_instruction(&mut machine, 0x633A);
    decode_call_instruction(&mut machine, 0x641A);

    //Draw A in the V2, V3 position (5 bits tall) (3A,1A)
    //decode_call_instruction(&mut machine, 0xD235);

    //PRUEBAS CON LA PANTALLA
    /* 
    let mut screen = Screen::new();

    screen.update();


    screen.flip_pixel(0,0);
    screen.flip_pixel(2,2);
    screen.flip_pixel(3,3);
    screen.flip_pixel(4,4);
    screen.flip_pixel(5,5);
    screen.flip_pixel(6,6);
    */



    let cpu_tick_counter = Arc::new(AtomicU8::new(0));
    let tick_counter = Arc::new(AtomicU32::new(0));
    let clock_counter = Arc::clone(&tick_counter);
    let delay_timer_counter = Arc::clone(&tick_counter);
    let sound_timer_counter = Arc::clone(&tick_counter);

    //thread::spawn(move|| util::clock::clock(clock_counter));

    //thread::spawn(move|| util::timers::delay_timer(delay_timer_counter));

    //thread::spawn(move|| util::timers::sound_timer(sound_timer_counter));

    //PRUEBA instrucciones

    //run(cpu_tick_counter);

    

    thread::sleep(Duration::from_secs(10));
    
}
