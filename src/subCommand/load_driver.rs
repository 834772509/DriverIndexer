use crate::utils::util::{getFileList};
use crate::utils::Zip7z::Zip7z;
use std::path::PathBuf;
use std::{fs};
use crate::TEMP_PATH;
use crate::utils::console::{writeConsole, ConsoleType};
use crate::subCommand::create_index::{getIndexData, InfInfo, getMatchInfo};
use crate::utils::devcon::Devcon;

/// 加载驱动包。支持驱动包路径、驱动路径
/// 参数1: 驱动包路径
/// 参数2: 索引Option
/// 参数3: 是否为精确匹配
pub fn loadDriver(driverPackPath: &PathBuf, indexPath: Option<PathBuf>, driveClass: Option<&str>, isAccurateMatch: bool) {
    let zip = Zip7z::new().unwrap();

    // 创建临时目录
    if !TEMP_PATH.exists() { fs::create_dir(&*TEMP_PATH).unwrap(); }

    writeConsole(ConsoleType::Info, &*format!("Load driver package: {}", driverPackPath.to_str().unwrap()));

    let mut infInfoList = Vec::new();

    // 当前临时驱动解压路径
    // let driversPath = TEMP_PATH.join(driverPackPath.file_stem().unwrap());
    let driversPath;

    if indexPath.is_some() {
        // ==========索引法==========
        driversPath = TEMP_PATH.join(driverPackPath.file_stem().unwrap());
        let mut indexPath = indexPath.unwrap();
        // 判断索引文件是否在驱动包内部（通过索引文件路径是否为相对路径）
        if indexPath.is_relative() {
            // 解压索引文件
            if !zip.extractFiles(&driverPackPath, &indexPath.to_str().unwrap(), &driversPath).unwrap() {
                writeConsole(ConsoleType::Err, "Failed to unzip the index file, please confirm whether the index file exists in the compressed package");
                return;
            };
            indexPath = driversPath.join(&indexPath);
        }
        // 解析索引文件
        infInfoList = match getIndexData(&indexPath) {
            Ok(infInfoList) => infInfoList,
            Err(_) => {
                writeConsole(ConsoleType::Err, "Index file parsing failed");
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
            writeConsole(ConsoleType::Err, "No driver detected in the driver package");
            return;
        }
        // 解析INF文件
        for item in infList.iter() {
            infInfoList.push(InfInfo::parsingInfFile(&driversPath, item).unwrap());
        }
        // println!("{:#?}", infInfoList);
    }

    // 匹配硬件id
    let matchHardwareAndDriver = getMatchInfo(&infInfoList, driveClass, isAccurateMatch).unwrap();
    if matchHardwareAndDriver.len() == 0 {
        writeConsole(ConsoleType::Err, "The driver that currently needs to be matched is not found");
        return;
    }

    // 循环匹配id
    for (hardware, infInfo) in matchHardwareAndDriver.iter() {
        // 当前状态：一个设备中有一个或多个驱动

        // 遍历匹配的驱动
        for infInfoItem in infInfo {
            writeConsole(ConsoleType::Info, &*format!("Matched to the driver: {}", &infInfoItem.Inf));

            // 获取解压路径（相对于解压所有inf文件的路径）
            let extractPath = &infInfoItem.Path;
            if driverPackPath.is_file() {
                // 解压匹配的驱动
                if !zip.extractFilesFromPath(&driverPackPath, extractPath.as_str(), &driversPath).unwrap() {
                    writeConsole(ConsoleType::Err, &*format!("Driver unzip failed, please make sure that the compressed package contains a layer of directories"));
                    continue;
                };
                writeConsole(ConsoleType::Info, &*format!("Driver unzip successfully"));
            }

            // 获取驱动路径
            let drivePath = &driversPath.join(&extractPath);

            // 获取驱动INF路径
            let driveInfPath = drivePath.join(&infInfoItem.Inf);

            // 获取HwID
            let hwid = hardware.HardwareIDs.get(0).unwrap_or(hardware.CompatibleIDs.get(0).unwrap());
            // 加载驱动
            if !Devcon::new().unwrap().loadDriver(&driveInfPath, hwid).unwrap() {
                writeConsole(ConsoleType::Err, &*format!("Driver installation failed"));
                // 如果驱动加载失败则加载下一驱动
                continue;
            }
            writeConsole(ConsoleType::Success, &*format!("Driver installed successfully"));
            // 驱动加载成功，进入下一设备的安装
            break;
        }
    }
}
