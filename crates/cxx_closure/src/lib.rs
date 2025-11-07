/*
* Example taken from cxx github page
*
* The DynFun approach is taken from:
* https://github.com/dtolnay/cxx/issues/114
*
* */

// This trait represents a thin wrapper of the "Fn" trait, 
// required to conform with the orphan rule.
pub trait Fun {
    fn execute(&self);
}

impl<F> Fun for F
where
    F: Fn(),
{
    fn execute(&self) {
        self()
    }
}

pub type DynFun = Box<dyn Fun>;

fn execute_dyn_task(f: &DynFun) {
    f.execute()
}

pub struct CallbackContext {
    pub msg: String,
}

#[cxx::bridge]
pub mod ffi {
    extern "Rust" {
        type CallbackContext;
        type DynFun;

        fn execute_dyn_task(f: &DynFun);
    }

    unsafe extern "C++" {
        include!("cxx-closure/include/closure.h");

        fn c_take_callback(
            callback: fn(),
        );


        fn c_take_callback_and_ctx(
            callback: fn(ctx: Box<CallbackContext>),
            ctx: Box<CallbackContext>,
        );

        fn c_take_task(
            callback: &DynFun,
        );
    }
}


