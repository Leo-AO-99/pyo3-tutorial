use pyo3::prelude::*;
use pyo3::exceptions::PyZeroDivisionError;
use std::num::ParseIntError;

#[pymodule(name = "math")]
pub fn _math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(parse_int, m)?)?;
    Ok(())
}

#[pyfunction]
fn fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[pyfunction]
fn divide(a: i32, b: i32) -> PyResult<i32> {
    if b == 0 {
        return Err(PyZeroDivisionError::new_err("b cannot be zero"));
    }
    Ok(a / b)
}

#[pyfunction]
fn parse_int(a: &str) -> Result<i32, ParseIntError> {
    a.parse::<i32>()
}