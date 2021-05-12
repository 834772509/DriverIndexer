fn main() {
    // 内置资源
    embed_resource::compile("./resource/resource.rc");

    // 声明绑定WinAPI
    windows::build!(
        Windows::Win32::Intl::GetUserDefaultUILanguage,

        // Windows::Win32::SystemServices::{PWSTR, BOOL},
        Windows::Win32::SystemServices::{NULL, INVALID_HANDLE_VALUE},
        Windows::Win32::DeviceAndDriverInstallation::{UpdateDriverForPlugAndPlayDevicesW},
        Windows::Win32::WindowsAndMessaging::HWND,

        Windows::Win32::DeviceAndDriverInstallation::*,
        Windows::Win32::Debug::GetLastError,

       Windows::Win32::SystemServices::{PWSTR,HANDLE},
       Windows::Win32::SystemServices::INVALID_HANDLE_VALUE,
    );
}
