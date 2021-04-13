use encoding::{DecoderTrap, Encoding};
use std::error::Error;
use std::process::Command;
use encoding::all::GBK;
use std::path::PathBuf;
use std::{env};
use std::fs::{File, OpenOptions};
use std::io::Write;
use crate::{Asset};
use glob::{MatchOptions};
use std::cmp::Ordering;

/// 写到文件
pub fn writeEmbedFile(filePath: &str, outFilePath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = Asset::get(filePath).unwrap();
    File::create(outFilePath)?.write(&file)?;
    Ok(())
}

/// 遍历目录及子目录下的所有指定文件
/// 参数1: 目录路径
/// 参数2: 文件通配符 如 *.inf
pub fn getFileList(path: &PathBuf, fileType: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut fileList: Vec<PathBuf> = Vec::new();
    // for item in WalkDir::new(path) {
    //     let item = item?;
    //     let extension = item.path().extension().unwrap_or(OsStr::new(""));
    //     if extension.to_str().unwrap().to_lowercase() == "inf".to_lowercase() {
    //         infList.push(item.path().to_path_buf());
    //     }
    // }

    let srerch = glob::glob_with(&*format!(r"{}\**\{}", path.to_str().unwrap(), fileType), MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    })?;
    for item in srerch {
        let path = PathBuf::from(&item.unwrap());
        if path.is_file() { fileList.push(path); }
    }
    Ok(fileList)
}

/// 获取7zip的程序路径
pub fn get7zProgramPath() -> Option<PathBuf> {
    let ProgramFiles: PathBuf = PathBuf::from(env::var("ProgramFiles").unwrap());
    let windir: PathBuf = PathBuf::from(env::var("windir").unwrap());

    let mayPath: [PathBuf; 6] = [PathBuf::from(r"D:\WimBuilder2-Full\bin\x64"), PathBuf::from(""), ProgramFiles.join(r"7-Zip"), ProgramFiles.join(r"7-Zip_x64"), ProgramFiles.join(r"7-Zip_x32"), windir.join(r"System32")];

    for item in mayPath.iter() {
        let zipPath = item.join("7z.exe");
        if zipPath.exists() {
            return Some(zipPath);
        }
    };
    None
}

/// 是否为压缩包文件
pub fn isArchive(archivePath: &PathBuf) -> bool {
    let extension = archivePath.extension().unwrap().to_str().unwrap_or("");
    let supportExtension = ["7z", "zip", "rar", "cab", "tar", "wim"];
    for item in supportExtension.iter() {
        if extension.to_lowercase() == *item.to_lowercase() {
            return true;
        }
    }
    return false;
}

/// 加载驱动-通过PECMD
pub fn loadDriverFromPECMD(driverPath: &PathBuf) -> Result<bool, Box<dyn Error>> {
    // PECMD TEAM DEVI *SUB %Temp%
    let _output = Command::new("PECMD.exe")
        .arg("DEVI")
        .arg("*SUB")
        .arg(driverPath)
        .output()?;
    Ok(true)
}

/// 加载驱动-通过Drvload
pub fn loadDriverFromDrvload(driverPath: &PathBuf) -> Result<bool, Box<dyn Error>> {
    let output = Command::new("drvload.exe")
        .arg(driverPath)
        .output()?;
    let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;
    Ok(content.contains("成功") || content.contains("success"))
}

/// 写日志
pub fn writeLogFile(logPath: &PathBuf, content: &String) -> Result<(), Box<dyn Error>> {
    // 尝试创建文件
    if !logPath.exists() { File::create(logPath).expect("无法创建日志文件"); }
    // 以追加模式打开文件
    let mut file = OpenOptions::new().append(true).open(logPath)?;
    file.write_all(format!("{}\r\n", content).as_bytes())?;
    Ok(())
}

/// 取出字符串左边文本
pub fn getStringLeft(content: &String, right: &str) -> Result<String, Box<dyn Error>> {
    let endSize = content.find(right).ok_or("发生错误-查找结束位置失败".to_owned())?;
    Ok((&content[..endSize]).to_string())
}

/// 取出字符串中间文本
pub fn getStringCenter(content: &String, start: &str, end: &str) -> Result<String, Box<dyn Error>> {
    let startSize = content.find(start).ok_or("发生错误-查找起始位置失败".to_owned())?;
    let endSize = startSize + content[startSize..].find(end).ok_or("发生错误-查找结束位置失败".to_owned())?;
    Ok((&content[startSize + start.len()..endSize]).to_string())
}

/// 取出字符串右边文本
pub fn getStringRight(content: &String, left: &str) -> Result<String, Box<dyn Error>> {
    let startSize = content.find(left).ok_or("发生错误-查找左边位置失败".to_owned())?;
    Ok((&content[startSize + left.len()..]).to_string())
}

/// 比较版本号大小
pub fn compareVersiopn(version1: &str, version2: &str) -> Ordering {
    let nums1: Vec<&str> = version1.split('.').collect();
    let nums2: Vec<&str> = version2.split('.').collect();
    let n1 = nums1.len();
    let n2 = nums2.len();

    // 比较版本
    for i in 0..std::cmp::max(n1, n2) {
        let i1 = if i < n1 { nums1[i].parse::<i32>().unwrap() } else { 0 };
        let i2 = if i < n2 { nums2[i].parse::<i32>().unwrap() } else { 0 };
        if i1 != i2 {
            return if i1 > i2 { Ordering::Greater } else { Ordering::Less };
        }
    }
    // 版本相等
    return Ordering::Equal;
}
