use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::cli::validator::{
    isValidDirectory, isValidDriverClass, isValidPath, isValidPathIncludeWildcard,
};
use crate::i18n::getLocaleText;

pub const HELP: &str = "help";
pub const DRIVEPATH: &str = "DrivePath";
pub const INDEXATH: &str = "IndexPath";
pub const DRIVECLASS: &str = "DriveClass";
pub const ALLDDEVICE: &str = "AllDevice";
pub const EXTRACTDRIVER: &str = "ExtractDriver";

pub fn cli<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        // 基本配置
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        // 模板
        .template(&*getLocaleText("template", None))
        .help_message(&*Box::leak(
            getLocaleText("help-message", None).into_boxed_str(),
        ))
        .version_message(&*Box::leak(
            getLocaleText("version-message", None).into_boxed_str(),
        ))
        .help_short("H")
        .subcommand(SubCommand::with_name(HELP)
            .help_short("H")
            .about(&*getLocaleText("help", None))
        )

        // Debug 模式
        .arg(
            Arg::with_name("debug")
                .short("D")
                .long("debug")
                .help(&*getLocaleText("on-debug", None)),
        )
        // 创建索引
        .subcommand(
            SubCommand::with_name("create-index")
                .about(&*getLocaleText("create-index", None))
                .help_short("H")
                .arg(
                    Arg::with_name(DRIVEPATH)
                        .value_name(DRIVEPATH)
                        .validator(isValidPath)
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name(INDEXATH)
                        .value_name(INDEXATH)
                        .index(2)
                        .help(&*getLocaleText("save-index-path", None)),
                ),
        )
        // 加载驱动
        .subcommand(
            SubCommand::with_name("load-driver")
                .about(&*getLocaleText("load-driver", None))
                .help_short("H")
                // 参数-驱动
                .arg(
                    Arg::with_name(DRIVEPATH)
                        .value_name(DRIVEPATH)
                        .validator(isValidPathIncludeWildcard)
                        .required(true)
                        .index(1)
                        .help(&*getLocaleText("package-path", None)),
                )
                // 参数-索引文件
                .arg(
                    Arg::with_name(INDEXATH)
                        .value_name(INDEXATH)
                        .index(2)
                        .help(&*getLocaleText("index-path", None)),
                )
                // 选项-匹配所有设备（包括已安装驱动设备）
                .arg(
                    Arg::with_name(ALLDDEVICE)
                        .short("a")
                        .long(ALLDDEVICE)
                        .help(&*getLocaleText("match-all-device", None)),
                )
                // 选项-驱动类别
                .arg(
                    Arg::with_name(DRIVECLASS)
                        .short("c")
                        .long(DRIVECLASS)
                        .value_name(DRIVECLASS)
                        .validator(isValidDriverClass)
                        .help(&*getLocaleText("driver-category", None)),
                )
                // 选项-仅解压不安装
                .arg(
                    Arg::with_name(EXTRACTDRIVER)
                        .short("e")
                        .long(EXTRACTDRIVER)
                        .value_name(EXTRACTDRIVER)
                        .help(&*getLocaleText("only-unzip", None)),
                ),
        )
        // 整理驱动
        .subcommand(
            SubCommand::with_name("classify-driver")
                .about(&*getLocaleText("classify-driver", None))
                .help_short("H")
                .arg(
                    Arg::with_name(DRIVEPATH)
                        .value_name(DRIVEPATH)
                        .validator(isValidDirectory)
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches()
}
