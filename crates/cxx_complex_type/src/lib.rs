#[cxx::bridge()]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-complex-type/include/matrixmult.hpp");

        // fn matrix_multiply(m: Vec<f64>, v: Vec<f64>) -> UniquePtr<CxxVector<f64>>;
        fn matrix_multiply(m: Vec<f64>, v: Vec<f64>) -> [f64; 3];
    }

}
