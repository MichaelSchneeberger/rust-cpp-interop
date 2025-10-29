use std::ffi::{CStr, CString};
use cmake_build::{cat, make_cat, cat_name, cat_meow, cat_feed};

fn main() {
    let marshmallow_name = CString::new("Marshmallow").unwrap();

    unsafe {
        let mut marshmallow = make_cat(marshmallow_name.as_ptr());
        println!(
            "Our cat is: {:?}",
            CStr::from_ptr(cat_name(&marshmallow as *const cat))
        );
        cat_meow(&marshmallow as *const cat);
        cat_feed(&mut marshmallow as *mut cat);
        cat_meow(&marshmallow as *const cat);
    }
}
