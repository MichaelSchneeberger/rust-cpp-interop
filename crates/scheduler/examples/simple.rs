use scheduler::ffi;

fn main() {
    let s = ffi::new_scheduler();
    ffi::schedule(s.clone(), |scheduler| {
        println!("hello");
        ffi::stop(scheduler);
    });
    ffi::start_loop(s);
    println!("done");
}
