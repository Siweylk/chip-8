use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

pub struct DelayTimer {
    delay_timer: u8,
}

impl DelayTimer {
    pub fn new() -> Self {
        Self { delay_timer: 60 }
    }

    pub fn delay_timer(&mut self, tick_counter: Arc<AtomicU8>) {
        let mut last_tick: u8 = 0;

        loop {
            //Get actual tick from clock
            let current_tick: u8 = tick_counter.load(Ordering::Relaxed);

            //If 10 clock cycles have passed, substract one from timer (clock 600Hz, timer 60Hz)
            if current_tick % 10 == 0 && current_tick != last_tick {
                last_tick = current_tick;
                self.delay_timer -= 1;
            }

            //Timer is at 0 --> do something
            if self.delay_timer == 0 {
                self.delay_timer = 60;
                //print!("Ha pasado 1 segundo\n");
            }
        }
    }

    pub fn set_delay_timer(&mut self, new_time: u8) {
        self.delay_timer = new_time;
    }

    pub fn get_delay_timer(&mut self) -> u8 {
        self.delay_timer
    }
}

pub struct SoundTimer {
    sound_timer: u8,
}

impl SoundTimer {
    pub fn new() -> Self {
        Self { sound_timer: 100 }
    }

    pub fn sound_timer(&mut self, tick_counter: Arc<AtomicU8>) {

        let mut last_tick: u8 = 0;

        loop {
            //Get actual tick from clock
            let current_tick: u8 = tick_counter.load(Ordering::Relaxed);

            //If 10 clock cycles have passed, substract one from timer (clock 600Hz, timer 60Hz)
            if current_tick % 10 == 0 && current_tick != last_tick {
                last_tick = current_tick;
                self.sound_timer -= 1;
            }

            //Timer is at 0 --> do something
            if self.sound_timer == 0 {
                self.sound_timer = 100;
                //print!("Han pasado 10 segundos\n");
            }
        }
    }

    pub fn set_sound_timer(&mut self, new_time: u8) {
        self.sound_timer = new_time;
    }
}
