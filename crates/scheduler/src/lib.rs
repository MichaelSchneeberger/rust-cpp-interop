#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("scheduler/include/scheduler.h");
        include!("scheduler/include/schedulershim.h");

        type Scheduler;

        fn new_scheduler() -> SharedPtr<Scheduler>;
        fn schedule(s: SharedPtr<Scheduler>, t: fn(SharedPtr<Scheduler>));
        fn schedule_with_ctx(s: SharedPtr<Scheduler>, t: fn(SharedPtr<Scheduler>), ctx: SharedPtr<Scheduler>);
        fn start_loop(s: SharedPtr<Scheduler>);
        fn stop(s: SharedPtr<Scheduler>);
    }
}

unsafe impl Send for ffi::Scheduler {}
unsafe impl Sync for ffi::Scheduler {}
