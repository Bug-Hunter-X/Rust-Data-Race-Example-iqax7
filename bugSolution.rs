use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let _thread1 = std::thread::spawn(move || {
        let mut data = y.lock().unwrap();
        *data = 10;
    });

    let _thread2 = std::thread::spawn(move || {
        let mut data = z.lock().unwrap();
        *data = 100; 
    });
    
    let final_value = x.lock().unwrap();
    println!("x = {}", *final_value);
}