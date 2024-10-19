use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyTuple;

#[pymodule(name = "signature")]
pub fn _signature(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kwds_args, m)?)?;
    m.add_function(wrap_pyfunction!(none_default, m)?)?;
    m.add_function(wrap_pyfunction!(args_args, m)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (**kwds))]
fn kwds_args(kwds: Option<&Bound<'_, PyDict>>) -> usize {
    kwds.map_or(0, |dict| dict.len())
}

#[pyfunction]
#[pyo3(signature = (*args))]
fn args_args(args: &Bound<'_, PyTuple>) {
    println!("args: {:?}", args);
}

#[pyfunction]
#[pyo3(signature = (n=None))]
fn none_default(n: Option<i32>) {
    match n {
        Some(n) => println!("n: {}", n),
        None => println!("n is None"),
    }
}