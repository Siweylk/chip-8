use std::thread;
use std::sync::{Arc, atomic::{AtomicU32, AtomicU8}};
use std::time::Duration;

use crate::util::cpu::{decode_call_instruction, run};
use crate::util::screen::{Screen};
use crate::util::machine::{Machine};

mod util;
mod assets;

fn main() {


    let cpu_tick_counter = Arc::new(AtomicU8::new(0));
    let tick_counter = Arc::new(AtomicU32::new(0));
    let clock_counter = Arc::clone(&tick_counter);
    let delay_timer_counter = Arc::clone(&tick_counter);
    let sound_timer_counter = Arc::clone(&tick_counter);

    //thread::spawn(move|| util::clock::clock(clock_counter));

    //thread::spawn(move|| util::timers::delay_timer(delay_timer_counter));

    //thread::spawn(move|| util::timers::sound_timer(sound_timer_counter));

    //PRUEBA instrucciones

    run(cpu_tick_counter);

    

    thread::sleep(Duration::from_secs(10));
    
}
