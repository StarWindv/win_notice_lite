use super::toast::Toast;
use pyo3::{PyResult, pyclass, pymethods};
use serde::Serialize;
use std::ops::Deref;

#[pyclass(from_py_object, get_all, set_all)]
#[derive(Serialize, Clone, Debug)]
pub struct MutableToast {
    id: u32,
    name: String,
    logo_uri: String,
    title: String,
    message: String,
    hero_image_uri: String,
    inline_images: Vec<String>,
    tag: String,
    group: String,
    creation_time: String,
    fingerprint: String,
    fingerprint_without_time: String,
}

impl Deref for MutableToast {
    type Target = Toast;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Toast) }
    }
}

#[pymethods]
impl MutableToast {
    #[new]
    pub fn __init__(
        id: u32,
        name: String,
        logo_uri: String,
        title: String,
        message: String,
        hero_image_uri: String,
        inline_images: Vec<String>,
        tag: String,
        group: String,
        creation_time: String,
        fingerprint: String,
        fingerprint_without_time: String,
    ) -> PyResult<Self> {
        Ok(Self {
            id,
            name,
            logo_uri,
            title,
            message,
            hero_image_uri,
            inline_images,
            tag,
            group,
            creation_time,
            fingerprint,
            fingerprint_without_time,
        })
    }
}
