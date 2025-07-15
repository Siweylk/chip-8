use std::time::Duration;
use std::thread;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};


pub fn clock(clock_counter: Arc<AtomicU32>){

    let frec: u64 = 1000000/600; //700 instructions per second (de momento)


    loop{
       
        clock_counter.fetch_add(1, Ordering::Relaxed);
        
        thread::sleep(Duration::from_micros(frec));
        

    }
}