// 禁用变量命名警告
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// 禁用未使用代码警告
#![allow(dead_code)]

#[macro_use]
mod macros;
mod cli;
mod i18n;
mod subCommand;
mod utils;

mod bindings {
    ::windows::include_bindings!();
}

#[cfg(test)]
mod tests;

#[macro_use]
extern crate clap;

use crate::utils::console::{writeConsole, ConsoleType};
use remove_dir_all::remove_dir_all;
use rust_embed::RustEmbed;
use std::env;
use std::path::PathBuf;

// 设置静态资源

// x64平台
#[cfg(target_arch = "x86_64")]
#[derive(RustEmbed)]
#[folder = "./assets-x64"]
pub struct Asset;

// x86平台
#[cfg(target_arch = "x86")]
#[derive(RustEmbed)]
#[folder = "./assets-x86"]
pub struct Asset;

// ARM平台
#[cfg(target_arch = "arm")]
#[derive(RustEmbed)]
#[folder = "./assets-ARM64"]
pub struct Asset;

// 设置静态变量
#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref TEMP_PATH: PathBuf =
        PathBuf::from(env::var("temp").unwrap()).join("DriverIndexer");
    pub static ref LOG_PATH: PathBuf =
        PathBuf::from(env::var("SYSTEMDRIVE").unwrap()).join(r"\Users\Log.txt");
}

fn main() {
    let matches = cli::cli::cli();
    cli::matches::matches(matches);
    // 清除临时目录
    if TEMP_PATH.exists() {
        if let Err(_e) = remove_dir_all(&*TEMP_PATH) {
            writeConsole(ConsoleType::Err, "Temporary directory deletion failed");
        }
    };
}
