use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{env, fs, thread};
use crate::i18n::getLocaleText;
use crate::subCommand::create_index::{InfInfo};
use crate::utils::console::{writeConsole, ConsoleType};
use crate::utils::devcon::{Devcon, HwID};
use crate::utils::sevenZIP::sevenZip;
use crate::utils::util::{compareVersiopn, getFileList};
use crate::utils::{newdevAPI, setupAPI};
use crate::TEMP_PATH;
use fluent_templates::fluent_bundle::FluentValue;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

/// 加载驱动包。支持驱动包路径、驱动路径
/// # 参数
/// 1. 驱动包路径
/// 2. 索引Option
/// 3. 是否为精确匹配
pub fn loadDriver(
    driverPackPath: &Path,
    indexPath: Option<PathBuf>,
    isAllDevice: bool,
    driveClass: Option<String>,
) {
    // 创建临时目录
    if !TEMP_PATH.exists() {
        fs::create_dir(&*TEMP_PATH).unwrap();
    }
    let zip = sevenZip::new().unwrap();
    let devcon = Devcon::new().unwrap();
    let setup = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"SYSTEM\Setup")
        .unwrap();

    let args: HashMap<String, FluentValue> =
        hash_map!("path".to_string() => driverPackPath.to_str().unwrap().into());
    writeConsole(
        ConsoleType::Info,
        &*getLocaleText("load-driver-package", Some(&args)),
    );

    let infInfoList;

    // 当前临时驱动解压路径
    let driversPath;

    if let Some(indexPath) = indexPath {
        // ==========索引法==========
        driversPath = TEMP_PATH.join(driverPackPath.file_stem().unwrap());
        // let mut indexPath = indexPath.unwrap();
        let indexPath = if indexPath.is_relative() {
            // 判断索引文件是否为相对路径
            let relativeIndexPath = driverPackPath.parent().unwrap().join(&indexPath);
            if relativeIndexPath.exists() { relativeIndexPath } else {
                // 尝试解压索引文件
                if !zip.extractFiles(&driverPackPath, &indexPath.to_str().unwrap(), &driversPath).unwrap() {
                    writeConsole(
                        ConsoleType::Err,
                        &*getLocaleText("unzip-index-failed", None),
                    );
                    return;
                };
                driversPath.join(&indexPath)
            }
        } else { indexPath };
        // 解析索引文件
        infInfoList = match InfInfo::parsingIndex(&indexPath) {
            Ok(infInfoList) => infInfoList,
            Err(_) => {
                writeConsole(
                    ConsoleType::Err,
                    &*getLocaleText("index-parsing-failed", None),
                );
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
        if infList.is_empty() {
            writeConsole(ConsoleType::Err, &*getLocaleText("no-driver-package", None));
            return;
        }
        // 多线程解析INF文件
        infInfoList = InfInfo::parsingInfFileList(&driversPath, &infList);
        // infInfoList: Vec<InfInfo> = infList.iter().map(|item| InfInfo::parsingInfFile(&basePath, item).unwrap()).collect();
    }

    let mut totalList: Vec<HwID> = Vec::new();

    // 3次匹配，避免部分驱动安装不全
    for scanCount in 0..3 {
        // 扫描以发现新的硬件
        // devcon.rescan().unwrap();
        unsafe {
            setupAPI::rescan();
        }
        // 获取真实硬件信息
        let mut hwIDList = devcon.getRealIdInfo(driveClass.clone()).unwrap();
        if hwIDList.is_empty() {
            writeConsole(ConsoleType::Err, &*getLocaleText("no-device", None));
            return;
        }
        // 判断是否需要获取有问题的硬件信息
        if !isAllDevice {
            hwIDList = devcon.getProblemIdInfo(hwIDList).unwrap();
        }

        // 过滤前一次安装的硬件信息
        let hwIDList: Vec<HwID> = hwIDList
            .clone()
            .into_iter()
            .filter(|item| !totalList.contains(item))
            .collect();
        // 硬件信息为空，当前没有需要安装驱动的设备
        if hwIDList.is_empty() {
            break;
        }
        for item in hwIDList.iter() {
            totalList.push(item.clone())
        }

        // 匹配硬件id
        let matchHardwareAndDriver = getMatchInfo(&hwIDList, &infInfoList).unwrap();
        if scanCount == 0 && matchHardwareAndDriver.is_empty() {
            writeConsole(
                ConsoleType::Err,
                &*getLocaleText("no-found-driver-currently", None),
            );
            break;
        }

        // 关闭驱动数字验证
        setup.set_value("SystemSetupInProgress", &(1_u32));

        // 任务列表
        let mut taskList = Vec::new();

        // 循环匹配信息
        for (hardware, infInfo) in matchHardwareAndDriver.iter() {
            // 当前状态：一个设备中有一个或多个驱动
            let driverPackPath = driverPackPath.to_path_buf();
            let driversPath = driversPath.clone();
            let hardware = hardware.clone();
            let infInfo = infInfo.clone();

            let task = thread::spawn(move || {
                match loadDriverPackage(&driverPackPath, &driversPath, &hardware, &infInfo) {
                    Ok(message) => {
                        writeConsole(ConsoleType::Success, &*message);
                    }
                    Err(error) => {
                        writeConsole(ConsoleType::Err, &*error);
                    }
                };
            });
            taskList.push(task);
        }

        // 等待所有线程执行完成
        taskList
            .into_iter()
            .map(|task| task.join())
            .collect::<Vec<_>>();

        // 恢复驱动数字验证
        setup.set_value("SystemSetupInProgress", &(0_u32));
    }
}

fn loadDriverPackage(
    driverPackPath: &Path,
    driversPath: &Path,
    hardware: &HwID,
    infInfo: &[InfInfo],
) -> Result<String, String> {
    lazy_static! {
        pub static ref ZIP: sevenZip = sevenZip::new().unwrap();
    }

    // 遍历匹配的驱动
    unsafe {
        for infInfoItem in infInfo.iter() {
            let arg: HashMap<String, FluentValue> = hash_map!(
                "class".to_string() => infInfoItem.Class.clone().into(),
                "deviceName".to_string() => hardware.Name.clone().into(),
                "deviceID".to_string() => infInfoItem.DriverList.get(0).unwrap_or(&"".to_string()).clone().into(),
                "driver".to_string() => infInfoItem.Inf.clone().into(),
                "version".to_string() => infInfoItem.Version.clone().into(),
            );

            // 获取解压路径（相对于解压所有INF文件的路径）
            let extractPath = &infInfoItem.Path;

            // 解压匹配的驱动
            if driverPackPath.is_file() && !ZIP.extractFilesFromPath(&driverPackPath, extractPath.as_str(), &driversPath).unwrap() {
                if Some(infInfoItem) != infInfo.last() {
                    continue;
                } else {
                    return Err(getLocaleText("install-message", Some(&arg)));
                }
            }

            // 获取INF路径
            let driveInfPath = driversPath.join(&extractPath).join(&infInfoItem.Inf);

            // 加载驱动
            let result: bool = infInfoItem
                .DriverList
                .iter()
                .map(|hwId| newdevAPI::updateDriverForPlugAndPlayDevices(&driveInfPath, hwId))
                .any(|x| x);
            // 如果当前驱动加载失败则加载下一驱动
            if !result {
                if Some(infInfoItem) != infInfo.last() {
                    continue;
                } else {
                    return Err(getLocaleText("install-message", Some(&arg)));
                }
            }

            return Ok(getLocaleText("install-message", Some(&arg)));
        }
        Err(getLocaleText("driver-install-failed", None))
    }
}

/// 获取匹配驱动的信息
/// # 参数
/// 1. 硬件ID列表
/// 2. INF驱动信息列表
/// # 规则
/// 1. 专用驱动优先级大于公版
/// 2. 高版本优先级大于低版本
pub fn getMatchInfo(
    idInfo: &[HwID],
    infInfoList: &[InfInfo],
) -> Result<Vec<(HwID, Vec<InfInfo>)>, Box<dyn Error>> {
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
            if !infInfoItem.Arch.contains(&currentArch.to_string()) {
                continue;
            }

            let mut matchInfInfo = InfInfo {
                DriverList: vec![],
                ..infInfoItem.clone()
            };
// 遍历INF中的硬件id
            let mut driverList: Vec<String> = infInfoItem
                .DriverList
                .clone()
                .into_iter()
                .filter(|infID| haID.to_lowercase() == infID.to_lowercase())
                .collect();
            matchInfInfo.DriverList.append(&mut driverList);
            if !matchInfInfo.DriverList.is_empty() {
                macthList.push(matchInfInfo.clone());
            }
        }
// 排序：高版本优先级大于低版本
        macthList.sort_by(|b, a| compareVersiopn(&*a.Version, &*b.Version));
        macthList
    };

// 匹配驱动信息
    let mut macthInfo: Vec<(HwID, Vec<InfInfo>)> = Vec::new();

// 遍历有问题的硬件id信息
    for idInfo in idInfo.iter() {
// 创建匹配信息列表
        let mut macthList: Vec<InfInfo> = Vec::new();

// 优先对比硬件id
        for haID in idInfo.HardwareIDs.iter() {
            macthList.append(&mut matchFn(haID));
        }

// 对比兼容id
        for haID in idInfo.CompatibleIDs.iter() {
            macthList.append(&mut matchFn(haID));
        }

// 没有匹配到该设备的驱动信息，则匹配下一个设备
        if macthList.is_empty() {
            continue;
        }

        macthInfo.push((idInfo.clone(), macthList));
    }
    Ok(macthInfo)
}
