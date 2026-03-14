use super::types::diff::Diff;
use super::types::listener::Listener;
use super::types::mutable_toast::MutableToast;
use super::types::toast::Toast;
use super::utils::parse_notification;

use std::collections::HashSet;

use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::{Bound, Python};
use pyo3::{PyResult, pymethods, pymodule};

use windows::UI::Notifications::Management::{
    UserNotificationListener, UserNotificationListenerAccessStatus,
};
use windows::UI::Notifications::{NotificationKinds, UserNotification};
use windows_collections::IVectorView;
use windows_future::IAsyncOperation;

#[pymodule]
fn win_notice_lite(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Listener>()?;
    m.add_class::<Toast>()?;
    m.add_class::<MutableToast>()?;
    m.add_class::<Diff>()?;
    Ok(())
}

#[pymethods]
impl Listener {
    /// 创建通知监听器实例
    ///
    /// Returns:
    ///
    ///     self (object): Result<Self> - 成功返回Listener实例, 失败返回错误
    #[new]
    pub fn new() -> PyResult<Self> {
        let listener = UserNotificationListener::Current().unwrap();
        Ok(Self { listener })
    }

    /// 请求通知访问权限 (提权)
    ///
    /// Notes:
    ///     建议从UI线程调用, 否则容易报错
    ///
    ///     但这是 C-Sharp 的规矩, 我也不知道转 Python 会发生什么, 大家用着看就是了
    ///
    /// Returns:
    ///
    ///     str: PyResult<String> -> IAsyncOperation<UserNotificationListenerAccessStatus>
    ///         也就是说拿到的就是权限字符串
    pub async fn elevate_privilege(&self) -> PyResult<String> {
        let operation: IAsyncOperation<UserNotificationListenerAccessStatus> =
            self.listener.RequestAccessAsync().unwrap();
        let status = operation.await.unwrap();
        Ok(format!("{:?}", status))
    }

    /// 获取当前系统中所有Toast类型的通知
    ///
    /// 逻辑:
    /// 1. 检查通知访问权限, 无权限直接返回空数组
    /// 2. 异步获取所有Toast类型通知, 解析为Toast结构体数组
    ///
    /// Returns:
    ///
    ///     list[Toast]: Result<Vec<Toast>> - 成功返回Toast数组, 失败返回Windows API错误
    pub async fn get_all_notifications(&self) -> PyResult<Vec<Toast>> {
        let status = self.listener.GetAccessStatus().unwrap();
        if status != UserNotificationListenerAccessStatus::Allowed {
            return Ok(vec![]);
        }

        let operation: IAsyncOperation<IVectorView<UserNotification>> = self
            .listener
            .GetNotificationsAsync(NotificationKinds::Toast)
            .unwrap();
        let raw_notifications = operation.await.unwrap();

        let mut notifications = Vec::with_capacity(raw_notifications.Size().unwrap() as usize);
        for i in 0..raw_notifications.Size().unwrap() {
            let notif = raw_notifications.GetAt(i).unwrap();
            notifications.push(parse_notification(&notif)?);
        }
        Ok(notifications)
    }

    /// 基于完整指纹 (含时间) 对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的指纹
    /// - 移除通知: 旧列表中有、新列表中无的指纹
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff[list[Toast], list[Toast]]: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_full(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint))
            .cloned()
            .collect();
        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于通知ID对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的ID
    /// - 移除通知: 旧列表中有、新列表中无的ID
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff[list[Toast], list[Toast]]: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_by_id(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_ids: HashSet<u32> = old.iter().map(|n| n.id.clone()).collect();
        let new_ids: HashSet<u32> = new.iter().map(|n| n.id.clone()).collect();

        let new_items: Vec<Toast> = new
            .into_iter()
            .filter(|n| !old_ids.contains(&n.id))
            .collect();

        let remove_items: Vec<Toast> = old
            .into_iter()
            .filter(|n| !new_ids.contains(&n.id))
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于无时间指纹对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的无时间指纹
    /// - 移除通知: 旧列表中有、新列表中无的无时间指纹
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_without_time(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint_without_time).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint_without_time).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();
        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 将通知列表序列化为格式化的 JSON 数组字符串
    ///
    /// 逻辑:
    /// - 使用serde_json序列化, 失败时返回空数组JSON字符串 ("[]")
    ///
    /// Arguments:
    ///
    ///     notifications (list[Toast]): &[Toast] - 待序列化的通知列表
    ///
    /// Returns:
    ///     str: 格式化的JSON字符串, 失败返回"[]"
    #[staticmethod]
    pub fn serialize(notifications: Vec<Toast>) -> String {
        serde_json::to_string_pretty(&notifications).unwrap_or_else(|_| "[]".to_string())
    }
}
