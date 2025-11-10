// This trait represents a thin wrapper of the "Fn" trait, 
// required to conform with the orphan rule.
// See https://github.com/dtolnay/cxx/issues/114
pub trait FunOnce {
    fn execute(self: Box<Self>);
}

impl<F> FunOnce for F
where
    F: FnOnce(),
{
    fn execute(self: Box<Self>) {
        (*self)()
    }
}

pub type DynFunOnce = Box<dyn FunOnce>;

fn execute_dyn_fun_once(f: Box<DynFunOnce>) {
    f.execute()
}

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type DynFunOnce;

        fn execute_dyn_fun_once(f: Box<DynFunOnce>);
    }

    unsafe extern "C++" {
        include!("scheduler/include/scheduler.h");
        include!("scheduler/include/schedulershim.h");

        type Scheduler;

        fn new_scheduler() -> SharedPtr<Scheduler>;
        fn schedule(s: SharedPtr<Scheduler>, t: Box<DynFunOnce>);
        // fn schedule_with_ctx(s: SharedPtr<Scheduler>, t: fn(SharedPtr<Scheduler>), ctx: SharedPtr<Scheduler>);
        fn start_loop(s: SharedPtr<Scheduler>);
        fn stop(s: SharedPtr<Scheduler>);
    }
}

pub fn schedule(s: cxx::SharedPtr<ffi::Scheduler>, task: impl FnOnce() + 'static) {
    let task_boxed = Box::new(task) as DynFunOnce;
    ffi::schedule(s, Box::new(task_boxed))
}


// https://github.com/dtolnay/cxx/issues/1175
unsafe impl Send for ffi::Scheduler {}
unsafe impl Sync for ffi::Scheduler {}

