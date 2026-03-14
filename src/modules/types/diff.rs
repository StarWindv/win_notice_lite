use super::toast::Toast;
use pyo3::pyclass;
use pyo3::{PyResult, pymethods};

///
/// Attributes:
///
///     new (list[Toast]): Vec<Toast>
///
///     remove (list[Toast]): Vec<Toast>
///
/// ### 说明:
/// **new**: 一个列表属性, 代表新增的通知, 直接访问即可
/// **remove**: 一个列表属性, 代表已去除的通知
#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone)]
pub struct Diff {
    pub new: Vec<Toast>,
    pub remove: Vec<Toast>,
}

#[pymethods]
impl Diff {
    #[new]
    pub fn __init__(new: Vec<Toast>, remove: Vec<Toast>) -> PyResult<Self> {
        Ok(Self { new, remove })
    }
}
