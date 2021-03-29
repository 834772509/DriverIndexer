use std::error::Error;
use crate::utils::util::{writeEmbedFile, getStringCenter, getStringRight, getStringLeft};
use std::process::Command;
use encoding::{DecoderTrap, Encoding};
use encoding::all::GBK;
use std::path::PathBuf;
use std::{fs};
use crate::TEMP_PATH;

/// 硬件信息
#[derive(Debug, Clone)]
pub struct HwID {
    /// 设备实例路径
    pub(crate) DeviceInstancePath: String,
    /// 显示名称
    pub(crate) Name: String,
    /// 硬件id
    pub(crate) HardwareIDs: Vec<String>,
    /// 兼容id
    pub(crate) CompatibleIDs: Vec<String>,
}


pub struct Devcon {
    devconPath: PathBuf,
}


impl Devcon {
    /// 初始化
    pub fn new() -> Result<Devcon, Box<dyn Error>> {
        if !TEMP_PATH.exists() { fs::create_dir(&*TEMP_PATH)?; }
        let devconPath = TEMP_PATH.join("devcon.exe");
        writeEmbedFile("devcon.exe", &devconPath)?;
        Ok(Devcon {
            devconPath
        })
    }

    /// 获取真实硬件id信息
    pub fn getRealIdInfo(&self) -> Result<Vec<HwID>, Box<dyn Error>> {
        let output = Command::new(&self.devconPath)
            .arg("hwids")
            .arg("*")
            .output()?;
        let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;

        // 将 Name 与 Hardware IDs 分离
        let content = content.replace("     Hardware IDs:", &*format!("\r\n    Hardware IDs:"));
        // 将 Name 与 Compatible IDs 分离，并加上空的Hardware IDs
        let content = content.replace("     Compatible IDs:", &*format!("\r\n    Hardware IDs:\r\n    Compatible IDs:"));

        const DELIMITER: &str = "|";
        const SUBDELIMITER: &str = ",";

        // 将输出的运行状态转换为每个项目一行以便读取
        let contentLine = content.replace("\r\n        ", SUBDELIMITER);
        let contentLine = contentLine.replace("\r\n    ", DELIMITER);
        let contentLine = contentLine.replace("  ", "");
        let contentLine = contentLine.replace("\r\n", &*format!("{}\r\n", DELIMITER));

        // let mut realIdList: Vec<String> = Vec::new();
        let mut HwIDList: Vec<HwID> = Vec::new();
        // 通过换行符分割遍历
        for item in contentLine.lines() {
            // realIdList.push(String::from(item));

            // 获取设备实例路径
            let DeviceInstancePath = getStringLeft(&item.to_string(), DELIMITER).unwrap_or("".to_string());

            // 获取显示名称
            let name = getStringCenter(&item.to_string(), "Name:", DELIMITER).unwrap_or("".to_string());

            // 获取硬件id
            let hardwareIDs = getStringCenter(&item.to_string(), "Hardware IDs:", DELIMITER).unwrap_or("".to_string()).replace(DELIMITER, "");
            let mut hardwareIDList: Vec<String> = Vec::new();
            for hardwareIdItem in hardwareIDs.split(SUBDELIMITER) {
                if hardwareIdItem != "" {
                    hardwareIDList.push(hardwareIdItem.to_string());
                }
            }

            // 获取兼容id
            let CompatibleIDs = getStringRight(&item.to_string(), "Compatible IDs:").unwrap_or("".to_string()).replace(DELIMITER, "");
            let mut CompatibleIDList: Vec<String> = Vec::new();
            for CompatibleIDItem in CompatibleIDs.split(SUBDELIMITER) {
                if CompatibleIDItem != "" {
                    CompatibleIDList.push(CompatibleIDItem.to_string());
                }
            }

            HwIDList.push(HwID {
                DeviceInstancePath,
                Name: name.trim().to_string(),
                HardwareIDs: hardwareIDList,
                CompatibleIDs: CompatibleIDList,
            });
        }
        Ok(HwIDList)
    }

    /// 获取有问题的硬件设备实例路径
    pub fn getProblemDeviceInstancePath(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // pnputil /enum-devices /problem /ids
        // 列出设备的运行状态
        let output = Command::new(&self.devconPath)
            .arg("status")
            .arg("*")
            .output()?;
        let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;

        const DELIMITER: &str = "|";

        // 将输出的运行状态转换为一行以便读取
        let contentLine = content.replace("\r\n    ", DELIMITER);

        let mut problemIdList: Vec<String> = Vec::new();
        // 通过换行符分割遍历
        for item in contentLine.lines() {
            if item.contains("problem") {
                let id = item.split(DELIMITER).next().unwrap_or("");
                if id != "" { problemIdList.push(String::from(id)); }
            }
        }
        Ok(problemIdList)
    }

    /// 获取有问题的硬件id信息
    pub fn getProblemIdInfo(&self) -> Result<Vec<HwID>, Box<dyn Error>> {
        let mut problemIdInfoList: Vec<HwID> = Vec::new();

        // 获取真实硬件id信息
        let realIdInfo = &self.getRealIdInfo()?;
        // 获取有问题的硬件设备实例路径
        let problemIdList = &self.getProblemDeviceInstancePath()?;

        // 遍历有问题的硬件设备实例路径
        for problemId in problemIdList {
            // 遍历获取真实硬件id信息
            for idInfo in realIdInfo.iter() {
                if problemId.to_lowercase() == idInfo.DeviceInstancePath.to_lowercase() {
                    problemIdInfoList.push(idInfo.clone());
                    break;
                }
            }
        }
        Ok(problemIdInfoList)
    }

    /// 加载驱动
    /// 注意hwid不是设备实例路径
    pub fn loadDriver(&self, infPath: &PathBuf, hwid: &String) -> Result<bool, Box<dyn Error>> {
        // 不要用 install 命令
        let output = Command::new(&self.devconPath)
            .arg("update")
            .arg(infPath)
            .arg(hwid)
            .output()?;
        let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;
        Ok(content.contains("successfully"))
    }

    /// 扫描以发现新的硬件
    pub fn rescan(&self) -> Result<bool, Box<dyn Error>> {
        let output = Command::new(&self.devconPath)
            .arg("rescan")
            .output()?;
        let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;
        Ok(content.contains("completed"))
    }

    /// 卸载设备
    pub fn removeDevice(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        let output = Command::new(&self.devconPath)
            .arg("remove")
            .arg(id)
            .output()?;
        let content = GBK.decode(&*output.stdout, DecoderTrap::Ignore)?;
        Ok(content.contains("were removed"))
    }
}
