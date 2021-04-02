use clap::{App, crate_version, AppSettings, SubCommand, Arg, ArgMatches};
use crate::utils::console::{writeConsole, ConsoleType};
use crate::subCommand;
use crate::utils::util::{getFileList};
use std::path::{PathBuf};
use crate::LOG_PATH;
use std::env;
use tokio::task;


const DRIVEPATH: &str = "DrivePath";
const INDEXATH: &str = "IndexPath";
// const NOINDEX: &str = "NoIndex";
const ACCURATEMATCH: &str = "AccurateMatch";

pub fn cli() -> App<'static, 'static> {
    App::new("DriverIndexer")
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())

        // .help_message("打印帮助信息")
        // .version_message("打印版本信息")
        //     .template(r"    {bin}
        // 用法：
        //     {usage}
        // 命令：
        // {subcommands}
        // 选项：
        // {flags}")

        .arg(Arg::with_name("debug")
            .short("D")
            .long("debug")
            .help("Turn on debug mode")
        )
        // .subcommand(SubCommand::with_name("help")
        //     .about("打印命令帮助。例：load-driver -help"))

        // 创建索引
        .subcommand(SubCommand::with_name("create-index")
            .about("Create a drive index. Index format: JSON")
            .arg(Arg::with_name(DRIVEPATH)
                .value_name(DRIVEPATH)
                .required(true)
                .index(1))
            .arg(Arg::with_name(INDEXATH)
                .value_name(INDEXATH)
                .required(true)
                .index(2))
        )

        // 加载驱动
        .subcommand(SubCommand::with_name("load-driver")
            .about("Install the matching driver. Automatically match the driver in the compressed package, decompress and install")
            .arg(Arg::with_name(DRIVEPATH)
                .value_name("Compressed package path")
                .required(true)
                .index(1))
            // 索引文件
            .arg(Arg::with_name(INDEXATH)
                .value_name(INDEXATH)
                .index(2))
            // 各种选项
            .arg(Arg::with_name(ACCURATEMATCH)
                .long(ACCURATEMATCH)
                .short("a")
                .help("Only match hardware id, not match compatible id"))
        )
        // .arg(Arg::with_name("extension")
        //     .long("extension")
        //     .short("e")
        //     .help("Do not check the suffix")))

        // 整理驱动
        .subcommand(SubCommand::with_name("classify-driver")
            .about("Sorting and sorting driver")
            .arg(Arg::with_name(DRIVEPATH)
                .value_name(DRIVEPATH)
                .required(true)
                .index(1)))
}

pub fn matches(matches: ArgMatches<'_>) {
    if isDebug() {
        writeConsole(ConsoleType::Info, &format!("Debug mode is open. The log is kept at {}", &LOG_PATH.to_str().unwrap()));
    }

    match matches.subcommand_name() {
        // 创建索引
        Some("create-index") => {
            let matches = matches.subcommand_matches("create-index").unwrap();

            let driverPath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());
            let indexPath = PathBuf::from(matches.value_of(INDEXATH).unwrap());
            subCommand::create_index::createIndex(&driverPath, &indexPath);
        }

        // 加载驱动
        Some("load-driver") => {
            let matches = matches.subcommand_matches("load-driver").unwrap();

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

                // let mut taskList = Vec::new();
                // 遍历驱动包
                for drivePathItem in driveList.iter() {
                    let index = indexIter.next().unwrap().clone();
                    subCommand::load_driver::loadDriver(drivePathItem, index, matches.is_present(ACCURATEMATCH));
                    // let task = subCommand::load_driver::loadDriver(drivePathItem, index, matches.is_present(ACCURATEMATCH));
                    // taskList.push(task);
                    // task::spawn(task);
                }
            } else {
                // 无通配符
                if drivePath.exists() == false || drivePath.is_dir() {
                    writeConsole(ConsoleType::Err, "The driver path is invalid, please enter the correct file path");
                    return;
                };

                let index = match matches.is_present(INDEXATH) {
                    true => Some(PathBuf::from(matches.value_of(INDEXATH).unwrap())),
                    false => None,
                };
                subCommand::load_driver::loadDriver(&drivePath, index, matches.is_present(ACCURATEMATCH));
            }
        }

        // 整理驱动
        Some("classify-driver") => {
            let matches = matches.subcommand_matches("classify-driver").unwrap();

            let inputPath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());
            if inputPath.is_dir() == false {
                writeConsole(ConsoleType::Err, "The path is invalid, please enter a valid path");
                return;
            }
            subCommand::classify_driver::classify_driver(&inputPath);
        }

        _ => {}
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
