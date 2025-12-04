use std::ops::FnOnce;

// This trait represents a thin wrapper of the "FnOnce" trait, 
// required to conform with the orphan rule:
// Traits (i.e., FnOnce()) defined outside the current crate cannot be implemented 
// for types (i.e., Box<dyn FnOnce()>) defined outside of the crate.
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

// pub type DynFunOnce = Box<dyn FunOnce>;
pub type DynFunOnce = Box<dyn FnOnce()>;

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

