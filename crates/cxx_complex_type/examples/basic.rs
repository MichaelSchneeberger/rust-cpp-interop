use cxx_complex_type::ffi;

fn main() {
    let m: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let v: Vec<f64> = vec![1.0, 0.0, 0.0];
    let r = ffi::matrix_multiply(m, v);
    println!("{:?}", r);
}
