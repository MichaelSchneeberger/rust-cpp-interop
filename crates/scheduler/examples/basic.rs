use scheduler::ffi;

fn main() {
    let s = ffi::new_scheduler();
    ffi::schedule(s.clone(), |inner_s| {
        println!("Scheduled task says hello");
        ffi::stop(inner_s);
    });
    ffi::start_loop(s);
}
