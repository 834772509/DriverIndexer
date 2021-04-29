use std::error::Error;
use std::path::PathBuf;
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
/// # 参数
/// 1. 目录路径
/// 2. 文件通配符 如 *.inf
pub fn getFileList(path: &PathBuf, fileType: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let srerch = glob::glob_with(&*format!(r"{}\**\{}", path.to_str().unwrap(), fileType), MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    })?;
    let fileList: Vec<PathBuf> = srerch.into_iter()
        .filter(|item| item.as_ref().unwrap().is_file())
        .map(|item| item.unwrap())
        .collect();
    Ok(fileList)
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

/// 写日志
pub fn writeLogFile(logPath: &PathBuf, content: &String) -> Result<(), Box<dyn Error>> {
    // 尝试创建文件
    if !logPath.exists() { File::create(logPath).expect("无法创建日志文件"); }
    // 以追加模式打开文件
    let mut file = OpenOptions::new().append(true).open(logPath)?;
    file.write_all(format!("{}\r\n", content).as_bytes())?;
    Ok(())
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

// 增加字符串截取方法
pub trait StringUtils {
    fn getStringLeft(&self, right: &str) -> Result<String, Box<dyn Error>>;
    fn getStringCenter(&self, start: &str, end: &str) -> Result<String, Box<dyn Error>>;
    fn getStringRight(&self, left: &str) -> Result<String, Box<dyn Error>>;
}

impl StringUtils for String {
    /// 取出字符串左边文本
    fn getStringLeft(&self, right: &str) -> Result<String, Box<dyn Error>> {
        let endSize = self.find(right).ok_or("发生错误-查找结束位置失败".to_owned())?;
        Ok((&self[..endSize]).to_string())
    }
    /// 取出字符串中间文本
    fn getStringCenter(&self, start: &str, end: &str) -> Result<String, Box<dyn Error>> {
        let startSize = self.find(start).ok_or("发生错误-查找起始位置失败".to_owned())?;
        let endSize = startSize + self[startSize..].find(end).ok_or("发生错误-查找结束位置失败".to_owned())?;
        Ok((&self[startSize + start.len()..endSize]).to_string())
    }
    /// 取出字符串右边文本
    fn getStringRight(&self, left: &str) -> Result<String, Box<dyn Error>> {
        let startSize = self.find(left).ok_or("发生错误-查找左边位置失败".to_owned())?;
        Ok((&self[startSize + left.len()..]).to_string())
    }
}
