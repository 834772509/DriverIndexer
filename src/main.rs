// 禁用变量命名警告
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// 禁用未使用代码警告
#![allow(dead_code)]

#[macro_use]
mod macros;
mod i18n;
mod subCommand;
mod utils;
mod cli;
mod test;

use std::path::{PathBuf};
use rust_embed::RustEmbed;
use std::{env};
use remove_dir_all::remove_dir_all;
use crate::utils::console::{writeConsole, ConsoleType};

// 设置静态资源
#[derive(RustEmbed)]
// #[cfg(target_arch = "x86_64")]
#[folder = "./assets-x64"]
pub struct Asset;

// #[derive(RustEmbed)]
// #[cfg(target_arch = "x86")]
// #[folder = "./assets-x86"]
// pub struct Asset;


// 设置静态变量
#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMP_PATH: PathBuf = PathBuf::from(env::var("temp").unwrap()).join("DriverIndexer");
    pub static ref LOG_PATH: PathBuf = PathBuf::from(env::var("SYSTEMDRIVE").unwrap()).join(r"\Users\Log.txt");
}

fn main() {
    let matches = cli::cli().get_matches();
    cli::matches(matches);
    // 清除临时目录
    if TEMP_PATH.exists() {
        if let Err(_e) = remove_dir_all(&*TEMP_PATH) { writeConsole(ConsoleType::Err, &*format!("Temporary directory deletion failed")); }
    };
}
