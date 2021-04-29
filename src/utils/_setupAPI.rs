use winapi::shared::guiddef::{GUID, LPGUID};
use winapi::_core::borrow::BorrowMut;

const CR_NO_SUCH_VALUE: u32 = 37;
const CR_SUCCESS: u32 = 0;
const MAX_DEV_LEN: u32 = 1000;
const MAX_PATH: u32 = 260;
const DIOCR_INSTALLER: u32 = 1;
const INVALID_HANDLE_VALUE: i32 = -1;


pub unsafe fn EnumDeviceClasses() -> Vec<GUID> {
    let mut nIndex = 0;
    let mut ClassGuid = GUID {
        Data1: 0,
        Data2: 0,
        Data3: 0,
        Data4: [0; 8],
    };

    let mut list = Vec::new();
    loop {
        let nRet = winapi::um::cfgmgr32::CM_Enumerate_Classes(nIndex, ClassGuid.borrow_mut(), 0);
        if nRet == CR_SUCCESS {
            list.push(ClassGuid);
            nIndex = nIndex + 1;
        } else if nRet == CR_NO_SUCH_VALUE { break; }
    }
    return list;
}

pub unsafe fn EnumDevices(ClassGuid: GUID) {
    let ClassName = GetClassName(ClassGuid);
    println!("{}", ClassName);
    let ClassDescription = GetClassDescription(ClassGuid);
}

unsafe fn GetClassName(mut ClassGuid: GUID) -> String {
    let mut RequiredSize = MAX_DEV_LEN;
    let mut ClassName: String = vec![' '; MAX_DEV_LEN as usize].iter().collect::<String>();
    winapi::um::setupapi::SetupDiClassNameFromGuidA(ClassGuid.borrow_mut(), ClassName.as_mut_ptr() as *mut i8, RequiredSize, RequiredSize.borrow_mut());
    if RequiredSize > 0 {
        ClassName = vec![' '; 1000].iter().collect::<String>();
        winapi::um::setupapi::SetupDiClassNameFromGuidA(ClassGuid.borrow_mut(), ClassName.as_mut_ptr() as *mut i8, RequiredSize as u32, RequiredSize.borrow_mut());
    }
    ClassName.trim().replace("\u{0}", "")
}

unsafe fn GetClassDescription(mut ClassGuid: GUID) -> String {
    let mut szClassDesc: String = vec![' '; MAX_PATH as usize].iter().collect::<String>();
    if winapi::um::setupapi::SetupDiGetClassDescriptionA(ClassGuid.borrow_mut(), szClassDesc.as_ptr() as *mut i8, MAX_PATH, 0.borrow_mut()) != 0 {
        return szClassDesc;
    }
    // let KeyClass = winapi::um::setupapi::SetupDiOpenClassRegKeyExA(ClassGuid.borrow_mut(), "sdf".parse().unwrap(), DIOCR_INSTALLER, "".as_mut_ptr() as *mut i8,std::os::raw::c_void::);
    // if KeyClass != INVALID_HANDLE_VALUE {
    //     let dwSize = MAX_DEV_LEN;
    //     let szClassDesc = vec![' '; dwSize as usize].iter().collect::<String>();
    //     // let nRet =
    // }
    return "".to_string();
}