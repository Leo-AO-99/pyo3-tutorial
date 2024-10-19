use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod math;
mod signature;

#[pymodule]
fn pyo3_tutorial(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    m.add_wrapped(wrap_pymodule!(math::_math))?;
    m.add_wrapped(wrap_pymodule!(signature::_signature))?;
    Ok(())
}

#[pyfunction]
#[pyo3(name = "sum_to_string")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn hello_world() {
    println!("Hello from Rust!");
}
