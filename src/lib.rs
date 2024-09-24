// lib.rs 是整个库的主文件
// 通常情况下，在lib.rs里整合各种Py Moudule/Py Package，在其他文件里去定义Py Module/Py Package中的成员
// 一个很流行的例子是pydantic-core: https://github.com/pydantic/pydantic-core

mod example_py_functions;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Py Module
#[pymodule]
fn python_rust_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 将Py Function绑定到Py Module上
    m.add_function(wrap_pyfunction!(example_py_functions::add_i32, m)?)?;
    m.add_function(wrap_pyfunction!(example_py_functions::dimensional_reduction, m)?)?;
    Ok(())
}