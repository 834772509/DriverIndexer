use crate::bindings::{
    Windows::Win32::DeviceAndDriverInstallation::{UpdateDriverForPlugAndPlayDevicesW},
    Windows::Win32::WindowsAndMessaging::HWND,
};
use std::path::PathBuf;

/// 更新驱动
/// [相关文档](https://docs.microsoft.com/zh-cn/windows/win32/api/newdev/nf-newdev-updatedriverforplugandplaydevicesa?redirectedfrom=MSDN)
/// # 参数
/// 1. INF路径
/// 2. 硬件ID
pub unsafe fn updateDriverForPlugAndPlayDevices(infPath: &PathBuf, hwid: String) -> bool {
    let mut isReboot = false.into();
    UpdateDriverForPlugAndPlayDevicesW(HWND(0), hwid, infPath.to_str().unwrap(), 0, &mut isReboot).as_bool()
}
