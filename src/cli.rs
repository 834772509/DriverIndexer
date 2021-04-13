use clap::{App, crate_version, AppSettings, SubCommand, Arg, ArgMatches};
use crate::utils::console::{writeConsole, ConsoleType};
use crate::subCommand;
use crate::utils::util::{getFileList};
use std::path::{PathBuf};
use crate::LOG_PATH;
use std::env;


const DRIVEPATH: &str = "DrivePath";
const INDEXATH: &str = "IndexPath";
const DRIVECLASS: &str = "DriveClass";
const PASSWORD: &str = "Password";

pub fn cli() -> App<'static, 'static> {
    // getLocaleText("on-debug", None).as_str()

    App::new("DriverIndexer")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())

        //     .help_message("打印帮助信息")
        //     .version_message("打印版本信息")
        //     .template(r"    {bin}
        // 用法：
        //     {usage}
        // 命令：
        //     {subcommands}
        // 选项：
        //     {flags}")

        // Debug 模式
        .arg(Arg::with_name("debug")
            .short("D")
            .long("debug")
            .help("Turn on debug mode")
        )

        // 创建索引
        .subcommand(SubCommand::with_name("create-index")
            .about("Create a drive index. Index format: JSON")
            .arg(Arg::with_name(DRIVEPATH)
                .value_name(DRIVEPATH)
                .validator(isValidPath)
                .required(true)
                .index(1))
            .arg(Arg::with_name(INDEXATH)
                .value_name(INDEXATH)
                .index(2)
                .help("save index file path"))
        )

        // 加载驱动
        .subcommand(SubCommand::with_name("load-driver")
                        .about("Install the matching driver. Automatically match the driver in the compressed package, decompress and install")
                        // 参数-驱动
                        .arg(Arg::with_name(DRIVEPATH)
                            .value_name(DRIVEPATH)
                            .validator(isValidPathIncludeWildcard)
                            .required(true)
                            .index(1)
                            .help("Compressed package path")
                        )
                        // 参数-索引文件
                        .arg(Arg::with_name(INDEXATH)
                            .value_name(INDEXATH)
                            .index(2))
                        // 选项-驱动类别
                        .arg(Arg::with_name(DRIVECLASS)
                            .short("c")
                            .long(DRIVECLASS)
                            .value_name(DRIVECLASS)
                            .validator(isValidDriverClass)
                            .help("Set the installed driver category")
                        )
                    // 选项-驱动包密码
                    // .arg(Arg::with_name(PASSWORD)
                    //     .short("p")
                    //     .long(PASSWORD)
                    //     .value_name(PASSWORD)
                    //     .help("Set driver package password")
                    // )
        )

        // 整理驱动
        .subcommand(SubCommand::with_name("classify-driver")
            .about("Sorting and sorting driver")
            .arg(Arg::with_name(DRIVEPATH)
                .value_name(DRIVEPATH)
                .validator(isValidDirectory)
                .required(true)
                .index(1))
        )
}

pub fn matches(matches: ArgMatches<'_>) {
    if isDebug() {
        writeConsole(ConsoleType::Info, &format!("Debug mode is open. The log is kept at {}", &LOG_PATH.to_str().unwrap()));
    }

    // 创建索引
    if let Some(matches) = matches.subcommand_matches("create-index") {
        let driverPath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());
        let indexPath = if matches.is_present(INDEXATH) {
            PathBuf::from(matches.value_of(INDEXATH).unwrap())
        } else {
            // 没有指定索引文件，使用默认索引文件名
            let indexName = format!("{}.index", driverPath.file_stem().unwrap().to_str().unwrap());
            PathBuf::from(driverPath.parent().unwrap().join(indexName))
        };

        subCommand::create_index::createIndex(&driverPath, &indexPath);
    }

    // 加载驱动
    if let Some(matches) = matches.subcommand_matches("load-driver") {
        let drivePath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());

        // 处理通配符
        let driveName = drivePath.file_name().unwrap().to_str().unwrap();
        if driveName.contains("*") || driveName.contains("?") {
            let driveList = getFileList(&PathBuf::from(&drivePath.parent().unwrap()), driveName).unwrap();
            if driveList.len() == 0 {
                writeConsole(ConsoleType::Err, "No driver package was found in this directory");
                return;
            }

            // 创建索引列表（无索引则使用None）
            let mut indexList: Vec<Option<PathBuf>> = Vec::new();
            if matches.is_present(INDEXATH) {
                let inedxPath = PathBuf::from(matches.value_of(INDEXATH).unwrap());
                let indexName = inedxPath.file_name().unwrap().to_str().unwrap();
                if indexName.contains("*") || indexName.contains("?") {
                    for item in getFileList(&PathBuf::from(&inedxPath.parent().unwrap()), indexName).unwrap() {
                        indexList.push(Some(item));
                    }
                } else {
                    indexList.push(Some(PathBuf::from(matches.value_of(INDEXATH).unwrap())));
                }
            } else {
                for _ in driveList.iter() {
                    indexList.push(None)
                }
            }

            let mut indexIter = indexList.iter();

            // 遍历驱动包
            for drivePathItem in driveList.iter() {
                let index = indexIter.next().unwrap().clone();
                subCommand::load_driver::loadDriver(drivePathItem, index, matches.value_of(DRIVECLASS));
            }
        } else {
            // 无通配符
            let index = match matches.is_present(INDEXATH) {
                true => Some(PathBuf::from(matches.value_of(INDEXATH).unwrap())),
                false => None,
            };
            subCommand::load_driver::loadDriver(&drivePath, index, matches.value_of(DRIVECLASS));
        }
    }

    // 整理驱动
    if let Some(matches) = matches.subcommand_matches("classify-driver") {
        let inputPath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());
        subCommand::classify_driver::classify_driver(&inputPath);
    }
}

pub fn isDebug() -> bool {
    // 调试环境
    if env::var("CARGO_PKG_NAME").is_ok() {
        return false;
    }
    let matches = cli().get_matches();
    return matches.is_present("debug");
}

/// 是否为有效的路径
pub fn isValidPath(path: String) -> Result<(), String> {
    let path = PathBuf::from(path);
    if path.exists() == false {
        return Err("The path does not exist, please make sure the entered directory exists".to_string());
    };
    Ok(())
}

/// 是否为有效的目录路径
pub fn isValidDirectory(directory: String) -> Result<(), String> {
    let path = PathBuf::from(directory);
    if path.exists() == false {
        return Err("The directory does not exist, please make sure the entered directory exists".to_string());
    };
    if !path.is_dir() {
        return Err("The path is not a directory, please make sure that the entered path is a directory".to_string());
    };
    Ok(())
}

/// 是否为有效的路径（包括通配符）
pub fn isValidPathIncludeWildcard(path: String) -> Result<(), String> {
    let path = PathBuf::from(path);

    let fileName = path.file_name().unwrap().to_str().unwrap();
    if fileName.contains("*") || fileName.contains("?") {
        return if path.parent().unwrap().exists() {
            Ok(())
        } else {
            Err("The path does not exist, please make sure the entered path exists".to_string())
        };
    }

    if !path.exists() {
        return Err("The path does not exist, please make sure the entered path exists".to_string());
    }

    Ok(())
}

/// 是否为有效的驱动类别
pub fn isValidDriverClass(class: String) -> Result<(), String> {
    // HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\{f8ecafa6-66d1-41a5-899b-66585d7216b7}
    let driverClass = ["XboxComposite", "RDPDR", "DXGKrnl", "RemotePosDevice", "DigitalMediaDevices", "PrintQueue", "WCEUSBS", "SecurityAccelerator", "HidMsr", "SystemRecovery", "vhdmp", "fvevol", "fvevol", "USB", "ContentScreener", "Media Center Extender", "PnpPrinters", "Dot4", "Replication", "Dot4Print", "CDROM", "Computer", "DiskDrive", "Display", "FDC", "HDC", "Keyboard", "MEDIA", "Modem", "Monitor", "Mouse", "MTD", "MultiFunction", "Net", "NetClient", "NetService", "NetTrans", "PCMCIA", "Ports", "Printer", "SCSIAdapter", "System", "Unknown", "FloppyDisk", "HidLineDisplay", "Processor", "MultiPortSerial", "Memory", "SmartCardReader", "Sensor", "VolumeSnapshot", "SmrDisk", "ScmDisk", "SmrVolume", "ScmVolume", "Biometric", "Proximity", "AudioProcessingObject", "OposLegacyDevice", "SoftwareComponent", "FSFilterSystem", "XLGuard", "SoftwareDevice", "PerceptionSimulation", "PhysicalQuotaManagement", "1394", "Infrared", "Image", "TapeDrive", "BasicDisplay", "Volume", "ContinuousBackup", "Battery", "HIDClass", "HidCashDrawer", "61883", "RdpVideoMiniport", "QuotaManagement", "NetDriver", "TS_Generic", "USBDevice", "CopyProtection", "LegacyDriver", "SmartCard", "EhStorSilo", "XLWFP", "SDHost", "Encryption", "rdpbus", "AntiVirus", "RDCamera", "ActivityMonitor", "USBFunctionController", "AVC", "AudioEndpoint", "BarcodeScanner", "WSDPrintDevice", "POSPrinter", "Camera", "RDPDR", "CFSMetadataServer", "MediumChanger", "SecurityEnhancer", "Miracast", "SBP2", "HSM", "Holographic", "XnaComposite", "SecurityDevices", "SmartCardFilter", "Bluetooth", "Extension", "Infrastructure", "UCM", "WPD", "ComputeAccelerator", "Firmware", "Compression", "Virtualization", "OpenFileBackup", "Undelete"];
    for item in driverClass.iter() {
        if item.to_lowercase() == class.to_lowercase() { return Ok(()); }
    }
    return Err("The driver category is incorrect, please enter the correct driver category".to_string());
}
