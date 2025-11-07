/*
* Example taken from cxx github page
*
* https://github.com/dtolnay/cxx/issues/114
* https://github.com/dtolnay/cxx/pull/85
* https://github.com/dtolnay/cxx/blob/6f132eee85461743fa048f1b79afc020d589f015/tests/ffi/tests.cc
*
* */

// This trait represents a thin wrapper of the "Fn" trait, 
// required to conform with the orphan rule.
pub trait Fun {
    fn execute(&self);
    // fn execute(self);
}

impl<F> Fun for F
where
    F: Fn(),
    // F: FnOnce(),
{
    fn execute(&self) {
    // fn execute(self) {
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
            // callback: DynFun,
        );
    }
}


