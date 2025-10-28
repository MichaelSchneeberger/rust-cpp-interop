use cxx_closure::{ffi, CallbackContext};

fn main() {
    ffi::c_take_callback(|ctx| {
        println!("{}", ctx.msg)
    }, Box::new(CallbackContext{msg: String::from("hello")}));
}
