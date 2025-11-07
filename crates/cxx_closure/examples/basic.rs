use cxx_closure::{ffi, CallbackContext, DynFun};

fn main() {
    ffi::c_take_callback(|| {println!("Hello from c_take_callback")});

    ffi::c_take_callback_and_ctx(|ctx| {
        println!("{}", ctx.msg)
    }, Box::new(CallbackContext{msg: String::from("Hello from c_take_callback_and_ctx")}));

    let msg = String::from("Hello from task");
    let task = Box::new(move || {println!("{}", msg)}) as DynFun;
    ffi::c_take_task(&task);
}
