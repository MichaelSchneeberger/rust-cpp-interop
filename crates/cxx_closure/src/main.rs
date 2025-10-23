/*
* Example taken from cxx github page
*
* https://github.com/dtolnay/cxx/pull/85
* https://github.com/dtolnay/cxx/blob/6f132eee85461743fa048f1b79afc020d589f015/tests/ffi/tests.cc
*
* */

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-closure/include/closure.h");

        fn c_take_callback(callback: fn(String) -> usize);
    }
}


fn main() {
    ffi::c_take_callback(|v: String| {
        println!("{}", v);
        0
    });
}
