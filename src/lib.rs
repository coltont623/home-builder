extern "C" {
    fn complex_math_operation(a: i64, b: i64) -> i64;
}

pub fn complex_math(a: i64, b: i64) -> i64 {
    unsafe { complex_math_operation(a, b) }
}
