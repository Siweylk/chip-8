

pub fn clock(frec: &i32){
    thread::sleep(Duration::from_secs(1/60));
}