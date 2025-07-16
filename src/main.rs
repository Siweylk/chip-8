use std::thread;
use std::sync::{Arc, atomic::{AtomicU32}};
use std::time::Duration;

use crate::util::screen::{Screen};

mod util;

fn main() {

    let tick_counter = Arc::new(AtomicU32::new(0));
    let clock_counter = Arc::clone(&tick_counter);
    let delay_timer_counter = Arc::clone(&tick_counter);
    let sound_timer_counter = Arc::clone(&tick_counter);

    thread::spawn(move|| util::clock::clock(clock_counter));

    thread::spawn(move|| util::timers::delay_timer(delay_timer_counter));

    thread::spawn(move|| util::timers::sound_timer(sound_timer_counter));

    //PRUEBA PANTALLA
    let mut screen = Screen::new();

    screen.update();
    screen.set_pixel(1,1);
    screen.set_pixel(2,2);
    screen.set_pixel(3,3);
    screen.set_pixel(4,4);
    screen.set_pixel(5,5);
    screen.set_pixel(6,6);
    



    //let rcv = rx.recv().unwrap();
    //println!("Main thread got {}", rcv);
    thread::sleep(Duration::from_secs(3));

    screen.update();

    screen.set_pixel(2,2);
    screen.set_pixel(4,4);
    screen.set_pixel(6,6);
    thread::sleep(Duration::from_secs(3));

    screen.update();

    thread::sleep(Duration::from_secs(10));
    
}
