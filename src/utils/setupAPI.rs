use crate::bindings::{
    Windows::Win32::DeviceAndDriverInstallation::SetupDiGetClassDevsW,
    Windows::Win32::WindowsAndMessaging::HWND,
    Windows::Win32::DeviceAndDriverInstallation::{DIGCF_ALLCLASSES},
    Windows::Win32::SystemServices::{NULL, INVALID_HANDLE_VALUE},
    Windows::Win32::Debug::GetLastError,
};
use windows::Guid;
use std::borrow::BorrowMut;

// https://docs.microsoft.com/zh-cn/windows-hardware/drivers/install/using-device-installation-functions

/// https://docs.microsoft.com/zh-cn/windows/win32/api/setupapi/nf-setupapi-setupdigetclassdevsexa
pub unsafe fn getDeviceInfo() {
    // println!("{:?}", Guid::zeroed().borrow_mut());
    // let mut ClassGuid = Guid::new().unwrap();
    // println!("{:?}", ClassGuid);

    // let hdevinfo = SetupDiGetClassDevsW(std::ptr::null_mut(), "", HWND(0), DIGCF_ALLCLASSES);
    let hdevinfo = SetupDiGetClassDevsW(std::ptr::null_mut(), "", HWND(0), DIGCF_ALLCLASSES);
    if hdevinfo == hdevinfo {
        println!("{:?}", GetLastError());
        return;
    }
    println!("{:?}", hdevinfo);
}
