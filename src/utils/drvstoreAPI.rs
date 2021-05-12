// [drvstore API 索引](https://github.com/strontic/strontic.github.io/blob/1e4b8bca9cc9bc152a4e82107a90fa7b2556fcb4/xcyclopedia/library/drvstore.dll-090C64B4CEBBB4527C64D8D8E7C637E9.md)
// [drvstore API 参数](https://github.com/WOA-Project/DriverUpdater/blob/0508f7ab731010361931fb9f704fd95caae53924/DriverUpdater/NativeMethods.cs)
// [drvstore API 示例](https://github.com/WOA-Project/DriverUpdater/blob/2a5b56bd16de18799a54b9d9a56676ac68f259ef/DriverUpdater/Program.cs)


use std::ptr::null;

pub unsafe fn DriverStoreOpen() {
    // internal static extern IntPtr DriverStoreOpenW(
    // string TargetSystemRoot,
    // string TargetSystemDrive,
    // uint Flags,
    // IntPtr Reserved);

    let lib = libloading::Library::new("drvstore.dll").unwrap();
    println!("{:?}", lib);
    let DriverStoreOpenW: libloading::Symbol<unsafe extern fn(&str, &str, usize, usize) -> usize> = lib.get(b"DriverStoreOpenW").unwrap();
    println!("{:?}", DriverStoreOpenW);

    let DevicePart = r"D:\Project\FirPE\Win10PE\DriverTest";
    let result = DriverStoreOpenW(&*format!(r"{}\Windows", DevicePart), DevicePart, 0, 0);
    println!("{:?}", result);
}
