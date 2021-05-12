use crate::cli::cli::{cli, ALLDDEVICE, DRIVECLASS, DRIVEPATH, INDEXATH};
use crate::i18n::getLocaleText;
use crate::subCommand;
use crate::utils::console::{writeConsole, ConsoleType};
use crate::utils::util::getFileList;
use crate::LOG_PATH;
use clap::ArgMatches;
use fluent_templates::fluent_bundle::FluentValue;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn matches(matches: ArgMatches<'_>) {
    if isDebug() {
        let arg: HashMap<String, FluentValue> =
            hash_map!("path".to_string() => LOG_PATH.to_str().unwrap().into());
        writeConsole(
            ConsoleType::Info,
            &*getLocaleText("opened-debug", Some(&arg)),
        );
    }

    // 创建索引
    if let Some(matches) = matches.subcommand_matches("create-index") {
        let driverPath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());
        let indexPath = if matches.is_present(INDEXATH) {
            PathBuf::from(matches.value_of(INDEXATH).unwrap())
        } else {
            // 没有指定索引文件，使用默认索引文件名: 驱动包名.index
            let indexName = format!(
                "{}.index",
                driverPath.file_stem().unwrap().to_str().unwrap()
            );
            driverPath.parent().unwrap().join(indexName)
        };

        subCommand::create_index::createIndex(&driverPath, &indexPath);
    }

    // 加载驱动
    if let Some(matches) = matches.subcommand_matches("load-driver") {
        let drivePath = PathBuf::from(matches.value_of(DRIVEPATH).unwrap());

        // 处理通配符
        let driveName = drivePath.file_name().unwrap().to_str().unwrap();
        if driveName.contains('*') || driveName.contains('?') {
            let driveList =
                getFileList(&PathBuf::from(&drivePath.parent().unwrap()), driveName).unwrap();
            if driveList.is_empty() {
                writeConsole(
                    ConsoleType::Err,
                    "No driver package was found in this directory",
                );
                return;
            }

            // 创建索引列表（无索引则使用None）
            let mut indexList: Vec<Option<PathBuf>> = Vec::new();
            if matches.is_present(INDEXATH) {
                let inedxPath = PathBuf::from(matches.value_of(INDEXATH).unwrap());
                let indexName = inedxPath.file_name().unwrap().to_str().unwrap();
                if indexName.contains('*') || indexName.contains('?') {
                    for item in getFileList(&PathBuf::from(&inedxPath.parent().unwrap()), indexName)
                        .unwrap()
                    {
                        indexList.push(Some(item));
                    }
                } else {
                    indexList.push(Some(PathBuf::from(matches.value_of(INDEXATH).unwrap())));
                }
            } else {
                indexList.append(
                    &mut driveList
                        .iter()
                        .map(|_item| None)
                        .collect::<Vec<Option<PathBuf>>>(),
                );
            }

            let mut indexIter = indexList.iter();

            // 遍历驱动包
            for drivePathItem in driveList.iter() {
                let index = indexIter.next().unwrap().clone();
                let class = matches.value_of(DRIVECLASS).map(|class| class.to_string());
                subCommand::load_driver::loadDriver(
                    drivePathItem,
                    index,
                    matches.is_present(ALLDDEVICE),
                    class,
                );
            }
        } else {
            // 无通配符
            let index = match matches.is_present(INDEXATH) {
                true => Some(PathBuf::from(matches.value_of(INDEXATH).unwrap())),
                false => None,
            };
            let class = matches.value_of(DRIVECLASS).map(|class| class.to_string());
            subCommand::load_driver::loadDriver(
                &drivePath,
                index,
                matches.is_present(ALLDDEVICE),
                class,
            );
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
    cli().is_present("debug")
}
