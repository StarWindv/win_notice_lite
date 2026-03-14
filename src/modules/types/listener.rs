use pyo3::pyclass;
use windows::UI::Notifications::Management::UserNotificationListener;

#[derive(Debug, Clone)]
#[pyclass(from_py_object)]
pub struct Listener {
    pub listener: UserNotificationListener,
}
