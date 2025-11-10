use scheduler::{schedule, ffi};

fn main() {
    let s = ffi::new_scheduler();
    let s_clone = s.clone();
    schedule(s.clone(), move || {
        println!("Scheduled task says hello");

        ffi::stop(s_clone);
    });
    ffi::start_loop(s);
}
