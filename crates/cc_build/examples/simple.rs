use std::os::raw::c_int;

use c_interface::simple_add_two_numbers;

pub unsafe extern "C" fn on_two_numbers_added(result: c_int) {
    println!("Got {}", result);
}

fn main() {
    let a = 1;
    let b = 2;

    println!("Adding {} and {}", a, b);

    unsafe {
        simple_add_two_numbers(1, 2, on_two_numbers_added);
    }
}
