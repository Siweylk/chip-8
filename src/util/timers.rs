use std::sync::{Arc, atomic::{AtomicU32, Ordering}};


pub fn delay_timer(tick_counter: Arc<AtomicU32>){
   
    let mut delay_timer:u32 = 60;

    let mut last_tick:u32 = 0;

    loop{
        
        //Get actual tick from clock
        let current_tick: u32 = tick_counter.load(Ordering::Relaxed);
        
        //If 10 clock cycles have passed, substract one from timer (clock 600Hz, timer 60Hz)
        if  current_tick % 10 == 0 && current_tick != last_tick{
            last_tick = current_tick;
            delay_timer -=1;
        }

        //Timer is at 0 --> do something
        if delay_timer==0{
            delay_timer=60;
            //print!("Ha pasado 1 segundo\n");
        }

    }
}

pub fn sound_timer(tick_counter: Arc<AtomicU32>){
     let mut sound_timer:u32 = 600;

    let mut last_tick:u32 = 0;

    loop{
        
        //Get actual tick from clock
        let current_tick: u32 = tick_counter.load(Ordering::Relaxed);
        
        //If 10 clock cycles have passed, substract one from timer (clock 600Hz, timer 60Hz)
        if  current_tick % 10 == 0 && current_tick != last_tick{
            last_tick = current_tick;
            sound_timer -=1;
        }

        //Timer is at 0 --> do something
        if sound_timer==0{
            sound_timer=600;
            //print!("Han pasado 10 segundos\n");
        }

    }
}