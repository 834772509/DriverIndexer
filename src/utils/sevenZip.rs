use std::process::{Command};
use std::path::PathBuf;
use std::error::Error;
use std::fs;
use crate::utils::util::writeEmbedFile;
use crate::TEMP_PATH;


pub struct Zip7z {
    zipProgram: PathBuf,
}

impl Zip7z {
    pub fn new() -> Result<Zip7z, Box<dyn Error>> {
        if !TEMP_PATH.exists() { fs::create_dir(&*TEMP_PATH)?; }
        let zipProgram = TEMP_PATH.join("7z.exe");
        writeEmbedFile("7z.exe", &zipProgram)?;
        writeEmbedFile("7z.dll", &TEMP_PATH.join("7z.dll"))?;
        Ok(Zip7z {
            zipProgram
        })
    }

    /// 7-zip 释放文件（指定压缩包内文件）
    /// 从存档中提取文件（不使用目录名）
    /// 注意：此命令会将压缩档案中的所有文件输出到同一个目录中
    /// 参数1: 压缩包路径
    /// 参数2: 解压路径
    /// 参数3: 输出路径
    pub fn extractFiles(&self, zipFile: &PathBuf, extractPath: &str, outPath: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let output = Command::new(&self.zipProgram)
            .arg("e")
            .arg(zipFile.to_str().unwrap())
            .arg(&extractPath)
            .arg("-y")
            .arg("-aos")
            .arg(format!("-o{}", outPath.to_str().unwrap()))
            .output()?;
        let content = String::from_utf8_lossy(&output.stdout);
        if !content.contains("No files to process") {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 7-zip 解压文件
    /// 提取具有完整路径的文件（保留文件路径）
    /// 参数1: 压缩包路径
    /// 参数2: 解压路径
    /// 参数3: 输出路径
    pub fn extractFilesFromPath(&self, zipFile: &PathBuf, extractPath: &str, outPath: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let output = Command::new(&self.zipProgram)
            .arg("x")
            .arg(zipFile.to_str().unwrap())
            .arg(if extractPath != "" { &extractPath } else { "*" })
            .arg("-y")
            .arg("-aos")
            .arg(format!("-o{}", outPath.to_str().unwrap()))
            .output()?;
        let outContent = String::from_utf8_lossy(&output.stdout);
        if outContent.contains("Everything is Ok") {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 7-zip 解压文件
    /// 提取具有完整路径的文件（递归子目录）
    /// 参数1: 压缩包路径
    /// 参数2: 解压路径
    /// 参数3: 输出路径
    pub fn extractFilesFromPathRecurseSubdirectories(&self, zipFile: &PathBuf, extractPath: &str, outPath: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let output = Command::new(&self.zipProgram)
            .arg("x")
            .arg("-r")
            .arg(zipFile.to_str().unwrap())
            .arg(&extractPath)
            .arg("-y")
            .arg("-aos")
            .arg(format!("-o{}", outPath.to_str().unwrap()))
            .output()?;
        let content = String::from_utf8_lossy(&output.stdout);
        if !content.contains("No files to process") {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
