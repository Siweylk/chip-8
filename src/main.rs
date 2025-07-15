use std::thread;
use std::sync::{Arc, atomic::{AtomicU32}};
use std::time::Duration;

mod util;

fn main() {

    let tick_counter = Arc::new(AtomicU32::new(0));
    let clock_counter = Arc::clone(&tick_counter);
    let delay_timer_counter = Arc::clone(&tick_counter);
    let sound_timer_counter = Arc::clone(&tick_counter);

    thread::spawn(move|| util::clock::clock(clock_counter));

    thread::spawn(move|| util::timers::delay_timer(delay_timer_counter));

    thread::spawn(move|| util::timers::sound_timer(sound_timer_counter));


    //let rcv = rx.recv().unwrap();
    //println!("Main thread got {}", rcv);
    thread::sleep(Duration::from_secs(1000));
    
}
