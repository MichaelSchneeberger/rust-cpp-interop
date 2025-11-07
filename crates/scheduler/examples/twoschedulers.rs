use std::thread;
use scheduler::ffi;

fn main() {
    let s1 = ffi::new_scheduler();
    let s2 = ffi::new_scheduler();

    let s1_cloned = s1.clone();
    ffi::schedule_with_ctx(s2.clone(), |s| {
        println!("Hello from s2");
        println!("Schedule task on s1");
        ffi::schedule(s, |s| {
            println!("Hello from s1");
            ffi::stop(s);
        })
    }, s1_cloned);

    thread::spawn(move || ffi::start_loop(s2));

    ffi::start_loop(s1);
}
