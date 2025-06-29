use std::thread;
use std::time::Duration;

mod clock;

fn main() {

    let frec = 1/60;

    thread::spawn(move||{
        for i in 1..1200{
            thread::sleep(Duration::from_secs(1/60));
        }
    });
    thread::sleep(Duration::from_millis(1));
    
}
