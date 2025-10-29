use std::ffi::c_char;

#[repr(C)]
pub struct cat {
    pub name_: *const c_char,
    pub is_sleepy: bool,
}

unsafe extern "C" {
    pub fn make_cat(name: *const c_char) -> cat;
    pub fn cat_name(this: *const cat) -> *const c_char;
    pub fn cat_feed(this: *mut cat);
    pub fn cat_meow(this: *const cat);
}
