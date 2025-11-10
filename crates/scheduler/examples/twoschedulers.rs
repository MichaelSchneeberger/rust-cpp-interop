use std::thread;
use scheduler::{ffi, schedule};

fn main() {
    let s1 = ffi::new_scheduler();
    let s2 = ffi::new_scheduler();

    let s1_clone = s1.clone();
    let s2_clone = s2.clone();
    schedule(s2.clone(), move || {
        println!("Hello from s2");
        println!("Schedule task on s1");
        schedule(s1_clone.clone(), move || {
            println!("Hello from s1");
            ffi::stop(s1_clone.clone());
            ffi::stop(s2_clone.clone());
        })
    });

    thread::spawn(move || ffi::start_loop(s2));

    ffi::start_loop(s1);
}
