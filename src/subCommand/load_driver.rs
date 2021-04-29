use crate::utils::util::{getFileList, compareVersiopn};
use crate::utils::sevenZip::sevenZip;
use std::path::PathBuf;
use std::{fs, thread, env};
use crate::TEMP_PATH;
use crate::utils::console::{writeConsole, ConsoleType};
use crate::subCommand::create_index::{parsingIndexData, InfInfo};
use crate::utils::devcon::{Devcon, HwID};
use crate::i18n::getLocaleText;
use fluent_templates::fluent_bundle::FluentValue;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

/// 加载驱动包。支持驱动包路径、驱动路径
/// # 参数
/// 1. 驱动包路径
/// 2. 索引Option
/// 3. 是否为精确匹配
pub fn loadDriver(driverPackPath: &PathBuf, indexPath: Option<PathBuf>, isAllDevice: bool, driveClass: Option<String>) {
    // 创建临时目录
    if !TEMP_PATH.exists() { fs::create_dir(&*TEMP_PATH).unwrap(); }
    let zip = sevenZip::new().unwrap();
    let devcon = Devcon::new().unwrap();
    let setup = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SYSTEM\Setup").unwrap();

    let args: HashMap<String, FluentValue> = hash_map!("path".to_string() => driverPackPath.to_str().unwrap().into());
    writeConsole(ConsoleType::Info, &*getLocaleText("load-driver-package", Some(&args)));

    let infInfoList;

    // 当前临时驱动解压路径
    let driversPath;

    if indexPath.is_some() {
        // ==========索引法==========
        driversPath = TEMP_PATH.join(driverPackPath.file_stem().unwrap());
        let mut indexPath = indexPath.unwrap();
        // 判断索引文件是否在驱动包内部（通过索引文件路径是否为相对路径）
        if indexPath.is_relative() {
            // 解压索引文件
            if !zip.extractFiles(&driverPackPath, &indexPath.to_str().unwrap(), &driversPath).unwrap() {
                writeConsole(ConsoleType::Err, &*getLocaleText("unzip-index-failed", None));
                return;
            };
            indexPath = driversPath.join(&indexPath);
        }
        // 解析索引文件
        infInfoList = match parsingIndexData(&indexPath) {
            Ok(infInfoList) => infInfoList,
            Err(_) => {
                writeConsole(ConsoleType::Err, &*getLocaleText("index-parsing-failed", None));
                return;
            }
        };
    } else {
        // ==========无索引法==========
        if driverPackPath.is_file() {
            driversPath = TEMP_PATH.join(driverPackPath.file_stem().unwrap());
            // 解压INF文件
            zip.extractFilesFromPathRecurseSubdirectories(&driverPackPath, "*.inf", &driversPath).unwrap();
        } else {
            // 驱动包为文件夹
            driversPath = PathBuf::from(driverPackPath);
        }
        let infList = getFileList(&driversPath, "*.inf").unwrap();
        if infList.len() == 0 {
            writeConsole(ConsoleType::Err, &*getLocaleText("no-driver-package", None));
            return;
        }
        // 多线程解析INF文件
        infInfoList = InfInfo::parsingInfFileList(&driversPath, &infList);
        // infInfoList: Vec<InfInfo> = infList.iter().map(|item| InfInfo::parsingInfFile(&basePath, item).unwrap()).collect();
    }

    // 扫描以发现新的硬件
    devcon.rescan().unwrap();
    // 获取真实硬件信息
    let mut hwIDList = devcon.getRealIdInfo(driveClass.clone()).unwrap();
    // 判断是否需要获取有问题的硬件信息
    if !isAllDevice { hwIDList = devcon.getProblemIdInfo(hwIDList).unwrap(); }

    // 匹配硬件id
    let matchHardwareAndDriver = getMatchInfo(hwIDList, &infInfoList).unwrap();
    if matchHardwareAndDriver.len() == 0 {
        writeConsole(ConsoleType::Err, &*getLocaleText("no-found-driver-currently", None));
        return;
    }

    // 释放INF信息列表
    drop(&infInfoList);

    // 关闭驱动数字验证
    setup.set_value("SystemSetupInProgress", &(1 as u32));

    // 任务列表
    let mut taskList = Vec::new();

    // 循环匹配信息
    for (hardware, infInfo) in matchHardwareAndDriver.iter() {
        // 当前状态：一个设备中有一个或多个驱动
        let driverPackPath = driverPackPath.clone();
        let driversPath = driversPath.clone();
        let hardware = hardware.clone();
        let infInfo = infInfo.clone();

        let task = thread::spawn(move || {
            match loadDriverPackage(&driverPackPath, &driversPath, &hardware, &infInfo) {
                Ok(message) => { writeConsole(ConsoleType::Success, &*message); }
                Err(error) => { writeConsole(ConsoleType::Err, &*error); }
            };
        });
        taskList.push(task);
    }

    // 等待所有线程执行完成
    taskList.into_iter().map(|task| task.join()).collect::<Vec<_>>();

    // 恢复驱动数字验证
    setup.set_value("SystemSetupInProgress", &(0 as u32));
    // }
}

fn loadDriverPackage(driverPackPath: &PathBuf, driversPath: &PathBuf, hardware: &HwID, infInfo: &Vec<InfInfo>) -> Result<String, String> {
    lazy_static! {
        pub static ref ZIP: sevenZip = sevenZip::new().unwrap();
        pub static ref DEVCON: Devcon = Devcon::new().unwrap();
    }

    // 遍历匹配的驱动
    for infInfoItem in infInfo.iter() {
        // 获取解压路径（相对于解压所有INF文件的路径）
        let extractPath = &infInfoItem.Path;

        // 解压匹配的驱动
        if driverPackPath.is_file() && !ZIP.extractFilesFromPath(&driverPackPath, extractPath.as_str(), &driversPath).unwrap() {
            // writeConsole(ConsoleType::Err, &*getLocaleText("driver-unzip-failed", None));
            // writeConsole(ConsoleType::Info, &*getLocaleText("driver-unzip-success", None));
            continue;
        }

        // 获取驱动路径
        let drivePath = &driversPath.join(&extractPath);
        // 获取驱动INF路径
        let driveInfPath = drivePath.join(&infInfoItem.Inf);

        // 加载驱动
        let result: bool = infInfoItem.DriverList.iter()
            .map(|hwid| DEVCON.loadDriver(&driveInfPath, hwid).unwrap())
            .collect::<Vec<bool>>()
            .contains(&true);
        // let result = true;

        if !result {
            // 如果驱动加载失败则加载下一驱动
            continue;
            // return Err(getLocaleText("driver-install-failed", None));
        }
        let arg: HashMap<String, FluentValue> = hash_map!(
                "class".to_string() => infInfoItem.Class.clone().into(),
                "deviceName".to_string() => hardware.Name.clone().into(),
                "deviceID".to_string() => hardware.DeviceInstancePath.clone().into(),
                "driver".to_string() => infInfoItem.Inf.clone().into(),
                "version".to_string() => infInfoItem.Version.clone().into(),
        );
        // 驱动加载成功，跳出当前匹配的设备
        return Ok(getLocaleText("install-success", Some(&arg)));
    }
    Err(getLocaleText("driver-install-failed", None))
}

/// 获取匹配驱动的信息
/// # 参数
/// 1. 硬件ID列表
/// 2. INF驱动信息列表
/// # 规则
/// 1. 专用驱动优先级大于公版
/// 2. 高版本优先级大于低版本
pub fn getMatchInfo(idInfo: Vec<HwID>, infInfoList: &Vec<InfInfo>) -> Result<Vec<(HwID, Vec<InfInfo>)>, Box<dyn Error>> {
    // 提示：
    // 循环次数少的放在外层，减少内层变量的操作次数
    // 一个设备信息 对应 多个匹配驱动信息

    // 当前系统架构
    let currentArch = match env::consts::ARCH {
        "x86" => "NTx86",
        "x86_64" => "NTamd64",
        "arm" => "NTarm",
        _ => "",
    };

    // 闭包函数-匹配
    let matchFn = |haID: &String| {
        let mut macthList: Vec<InfInfo> = Vec::new();
        // 遍历INF信息列表
        for infInfoItem in infInfoList.iter() {
            // 如果INF不适用当前系统则进行匹配下一个INF
            if !infInfoItem.Arch.contains(&currentArch.to_string()) { continue; }
            let mut matchInfInfo = InfInfo { DriverList: vec![], ..infInfoItem.clone() };

            // 遍历INF中的硬件id
            let mut driverList: Vec<String> = infInfoItem.DriverList.clone().into_iter().filter(|infID| haID.to_lowercase() == infID.to_lowercase()).collect();
            matchInfInfo.DriverList.append(&mut driverList);
            if !matchInfInfo.DriverList.is_empty() { macthList.push(matchInfInfo.clone()); }
        }
        // 排序：高版本优先级大于低版本
        macthList.sort_by(|b, a| compareVersiopn(&*a.Version, &*b.Version));
        return macthList;
    };

    // 匹配驱动信息
    let mut macthInfo: Vec<(HwID, Vec<InfInfo>)> = Vec::new();

    // 遍历有问题的硬件id信息
    for idInfo in idInfo.iter() {
        // 创建匹配信息列表
        let mut macthList: Vec<InfInfo> = Vec::new();

        // 优先对比硬件id
        for haID in idInfo.HardwareIDs.iter() { macthList.append(&mut matchFn(haID)); }

        // 对比兼容id
        for haID in idInfo.CompatibleIDs.iter() { macthList.append(&mut matchFn(haID)); }

        // 没有匹配到该设备的驱动信息，则匹配下一个设备
        if macthList.len() == 0 { continue; }

        macthInfo.push((idInfo.clone(), macthList));
    }
    Ok(macthInfo)
}
