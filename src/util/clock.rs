use std::time::Duration;
use std::thread;


pub fn clock(frec: u64){


    let mut tic:i32 = 0;

    loop{

        thread::sleep(Duration::from_micros(frec));
        
        
        if tic != 60{
            tic += 1;
        }
        else{
            tic = 0;
            println!("Ha pasado un segundo");
        }

    }
}