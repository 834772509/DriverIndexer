use clap::{App, crate_version, AppSettings, SubCommand, Arg};

use crate::cli::validator::{isValidPath, isValidPathIncludeWildcard, isValidDriverClass, isValidDirectory};

pub const DRIVEPATH: &str = "DrivePath";
pub const INDEXATH: &str = "IndexPath";
pub const DRIVECLASS: &str = "DriveClass";
pub const PASSWORD: &str = "Password";
pub const ALLDDEVICE: &str = "AllDevice";

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
                        // 选项-匹配所有设备（包括已安装驱动设备）
                        .arg(Arg::with_name(ALLDDEVICE)
                            .short("a")
                            .long(ALLDDEVICE)
                            .help("Match all device")
                        )
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
