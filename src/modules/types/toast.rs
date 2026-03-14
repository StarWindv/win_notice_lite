use pyo3::{PyResult, pyclass, pymethods};
use serde::Serialize;

/// 代表单个通知的对象结构
///
/// Attributes:
///
///     id (int): u32 - 系统分配的通知标识符, 唯一 ID
///
///     name (str): String - 通知来源程序名称
///
///     logo_uri (str): String - 通知来源程序图标 URI - 实际为空
///
///     title (str): String - 通知标题
///
///     message (str): String - 通知内容
///
///     hero_image_uri (str): String - 通知主图 URI - 实际为空
///
///     inline_images(list[str]): Vec<String> - 通知内联图片的 URI 数组 - 实际为空
///
///     tag (str): String - 通知标签 - 实际为空
///
///     group (str): String - 通知分组 - 实际为空
///
///     creation_time (str): String - 通知创建时间
///
///     fingerprint (str): String - 通知的完整指纹
///
///     fingerprint_without_time (str): String - 不考虑时间戳的完整指纹
///
/// ### 时间说明
///
/// **creation_time**: 这玩意儿是`Windows`给的时间, 代表通知创建时的特殊时间戳, 开始于`1601/01/01 UTC`
///
/// ### 指纹说明
///
/// 指纹实际上是用空格作为分隔符 <br>
/// 去拼接除了`fingerprint/fingerprint_without_time`之外的所有字段 <br>
/// 然后求一个 sha2-256
///
/// ### 特殊实现说明 (因 Windows API 限制的降级处理)
///
/// 1. logo_uri: `AppDisplayInfo::GetLogo` 返回 `RandomAccessStreamReference`, 无直接`AbsoluteUri`属性, 故设为空字符串
/// 2. hero_image_uri/inline_images: `NotificationBinding`无`GetImageElements`方法, 且`AdaptiveNotificationImage`类型在`windows crate`中不存在, 故设为空
/// 3. tag/group: `Listener API`未暴露该字段 (仅发送通知时可设置) , 故设为空字符串
#[pyclass(from_py_object, get_all)]
#[derive(Serialize, Clone, Debug)]
pub struct Toast {
    pub id: u32,
    pub name: String,
    pub logo_uri: String,
    pub title: String,
    pub message: String,
    pub hero_image_uri: String,
    pub inline_images: Vec<String>,
    pub tag: String,
    pub group: String,
    pub creation_time: String,
    pub fingerprint: String,
    pub fingerprint_without_time: String,
}

#[pymethods]
impl Toast {
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
