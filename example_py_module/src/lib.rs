use pyo3::prelude::*;


/*
Some separately defined functions
 */

/// example py function
/// Add.
#[pyfunction]
pub fn add_i32(x: i32, y:i32) -> PyResult<i32> {
    return Ok( x + y );
}

/// example py function
/// Two-dimensional reduction to one-dimensional.
#[pyfunction]
pub fn dimensional_reduction(matrix: Vec<Vec<i32>>) -> PyResult<Vec<i32>> {
    let mut result_vec = Vec::new();

    for mut vec in matrix {
        result_vec.append(&mut vec);
    }

    Ok(result_vec)
}

/// A Python module implemented in Rust.
#[pymodule]
mod example_py_module {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pymodule_export]
    use super::add_i32;

    #[pymodule_export]
    use super::dimensional_reduction;
}
