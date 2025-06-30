use std::thread;
//use std::sync::mpsc;
use std::time::Duration;

mod util;

fn main() {

    let frec: u64 = 1000000/60;

    //let (tx,rx) = mpsc::channel();

    thread::spawn(move|| util::clock::clock(frec));

    //let rcv = rx.recv().unwrap();
    //println!("Main thread got {}", rcv);
    thread::sleep(Duration::from_secs(10));
    
}
