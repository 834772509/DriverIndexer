use std::path::PathBuf;
use std::error::Error;
use std::fs;
use encoding::{DecoderTrap};
use regex::{Regex};
use std::fs::File;
use std::io::Read;
use crate::utils::util::{getFileList, getStringCenter};
use crate::utils::console::{writeConsole, ConsoleType};
use serde::{Serialize, Deserialize};
use chardet::{detect, charset2encoding};
use encoding::label::encoding_from_whatwg_label;
use crate::utils::devcon::{HwID, Devcon};
use crate::TEMP_PATH;
use crate::utils::Zip7z::Zip7z;

/// INF驱动信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfInfo {
    /// 驱动路径
    pub(crate) Path: String,
    /// 驱动INF文件名
    pub(crate) Inf: String,
    /// 驱动类别
    pub(crate) Class: String,
    /// 驱动厂商
    pub(crate) Provider: String,
    /// 驱动硬件id列表
    pub(crate) DriverList: Vec<String>,
}

impl InfInfo {
    /// 解析inf文件
    /// 参数1: inf 基本路径（父路径）
    /// 参数2: inf 文件路径
    pub fn parsingInfFile(basePath: &PathBuf, infFile: &PathBuf) -> Result<InfInfo, Box<dyn Error>> {
        // let regExpression = [r"PCI\\.*?&.*?&[^; \f\n\r\t\v]+", r"USB\\.*?&[^; \f\n\r\t\v]+", ];
        // 所有类别取自 [HKLM\SYSTEM\ControlSet001\Enum]
        let Class = ["ACPI", "ACPI_HAL", "BTH", "BTHENUM", "BTHHFENUM", "DISPLAY", "HDAUDIO", "HID", "HTREE", "PCI", "ROOT", "SCSI", "STORAGE", "SW", "SWD", "TERMINPUT_BUS", "TS_USB_HUB_Enumerator", "UEFI", "UMB", "USB", "USBSTOR"];

        let mut regExpression = Vec::new();
        for item in Class.iter() {
            // 驱动ID匹配正则表达式
            // regExpression.push(format!(r",{}\\[^; \f\n\r\t\v]+", item));
            regExpression.push(format!(r",{}\\[^,; \f\n\r\t\v]+", item));
        }

        // 读取INF文件
        let mut file = File::open(&infFile)?;
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf)?;

        // 自动识别编码并以UTF-8读取
        let result = detect(&fileBuf);
        let coder = encoding_from_whatwg_label(charset2encoding(&result.0)).unwrap();
        let infContent = coder.decode(&fileBuf, DecoderTrap::Ignore)?;

        // 去除INF内的所有空格
        let infContent = infContent.replace(" ", "");
        // 去除INF内的所有tab符
        let infContent = infContent.replace("	", "");

        let mut idList: Vec<String> = Vec::new();

        // 遍历正则表达式
        for item in regExpression.iter() {
            let re = Regex::new(item)?;
            for iten in re.captures_iter(&*infContent) {
                // 将数据增加至数组，最后返回整个数组
                let id = String::from(iten.get(0).unwrap().as_str());
                // 删除逗号
                let id = id.replace(",", "");
                // 检测是否重复
                if !idList.contains(&id) {
                    idList.push(id);
                }
            }
        };

        // 获取INF类别
        let class = getStringCenter(&infContent, "Class=", "\n").unwrap_or("".to_string()).replace("\r", "");

        // 获取INF版本
        // let driverDateAndDriverVer = getStringCenter(&infContent, "DriverVer=", "\n").unwrap_or("".to_string());
        // println!("{:?}", driverDateAndDriverVer.split(","));

        // 获取INF厂商
        let provider = getStringCenter(&infContent, "Provider=", "\n").unwrap_or("".to_string())
            .replace("\r", "")
            .replace("%", "");
        let provider = getStringCenter(&infContent, &format!("{}=", provider), "\n").unwrap_or(provider)
            .replace("\r", "")
            .replace("\"", "")
            .replace("Corporation", "")
            .replace("SemiconductorCorp.", "")
            .replace("TechnologyCorp.", "")
            .replace(",Inc.", "")
            .replace("®", "")
            .replace("(R)", "");
        // if provider == "" {
        // println!("{:?}", infFile);
        // println!("{}", infContent);
        // }

        // 获取INF文件相对路径
        let parentPath = infFile.parent().unwrap().strip_prefix(basePath)?;

        Ok(InfInfo {
            Path: parentPath.to_str().unwrap().parse().unwrap(),
            Inf: infFile.file_name().unwrap().to_str().unwrap().parse().unwrap(),
            Class: class,
            Provider: provider,
            DriverList: idList,
        })
    }
}

pub fn createIndex(basePath: &PathBuf, saveIndexPath: &PathBuf) {
    writeConsole(ConsoleType::Info, "Processing, please wait……");

    let zip = Zip7z::new().unwrap();

    // INF文件父路径
    let infParentPath;
    // INF文件列表
    let infList;
    // 保存索引路径
    let indexPath;

    if basePath.is_dir() {
        // 从驱动目录中创建索引文件
        infList = getFileList(&basePath, "*.inf").unwrap();
        infParentPath = basePath.clone();
        // 如果输入的索引路径是相对路径，则令实际路径为驱动目录所在路径
        indexPath = if saveIndexPath.is_relative() { basePath.join(&saveIndexPath) } else { saveIndexPath.clone() };
    } else {
        // 从文件中创建索引文件
        infParentPath = TEMP_PATH.join(basePath.file_stem().unwrap());
        // 解压INF文件
        zip.extractFilesFromPathRecurseSubdirectories(&basePath, "*.inf", &infParentPath).unwrap();
        infList = getFileList(&infParentPath, "*.inf").unwrap();
        // 如果输入的索引路径是相对路径，则令实际实际为驱动包所在路径
        indexPath = if saveIndexPath.is_relative() { basePath.parent().unwrap().join(&saveIndexPath) } else { saveIndexPath.clone() };
    }

    let mut infInfoList: Vec<InfInfo> = Vec::new();
    let mut successCount = 0;
    let mut ErrorCount = 0;
    let mut blankCount = 0;

    // 遍历INF文件
    for item in infList.iter() {
        match InfInfo::parsingInfFile(&infParentPath, item) {
            Ok(currentInfo) => {
                if currentInfo.DriverList.len() == 0 {
                    blankCount += 1;
                    writeConsole(ConsoleType::Warning, &*format!("The hardware id in this file is not detected: {}", &item.to_str().unwrap()));
                    continue;
                }
                successCount += 1;
                infInfoList.push(currentInfo);
            }
            Err(_e) => {
                ErrorCount += 1;
                writeConsole(ConsoleType::Err, &*format!("INF parsing error: {}", &item.to_str().unwrap()));
            }
        };
    };

    if let Err(_e) = saveDataFromJson(&infInfoList, &indexPath) {
        writeConsole(ConsoleType::Err, "Failed to save index file");
        return;
    }
    writeConsole(ConsoleType::Info, &*format!("Total {} items，Processed {} items，{} items failed to process，{} items may not have hardware id information", &infList.len(), &successCount, &ErrorCount, &blankCount));
    writeConsole(ConsoleType::Info, &*format!("The drive index is saved in \"{}\"", &indexPath.to_str().unwrap()));
}


/// 保存inf数据（通过json）
fn saveDataFromJson(Data: &Vec<InfInfo>, savaPath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(&Data)?;
    fs::write(savaPath, json)?;
    Ok(())
}

/// 获取索引数据
/// 参数1: inf文件路径
pub fn getIndexData(indexPath: &PathBuf) -> Result<Vec<InfInfo>, Box<dyn Error>> {
    let mut indexFile = File::open(indexPath)?;
    let mut indexContent = String::new();
    indexFile.read_to_string(&mut indexContent)?;
    let json: Vec<InfInfo> = serde_json::from_str(&*indexContent)?;
    Ok(json)
}

/// 获取匹配驱动的信息
/// 参数1: INF驱动信息列表
/// 参数2: 是否为精准匹配（不匹配兼容ID）
pub fn getMatchInfo(infInfoList: &Vec<InfInfo>, isAccurateMatch: bool) -> Result<Vec<(HwID, Vec<InfInfo>)>, Box<dyn Error>> {
    let devcon = Devcon::new()?;
    // 扫描以发现新的硬件
    devcon.rescan()?;
    // 获取有问题的硬件id信息
    let idInfo = devcon.getProblemIdInfo()?;

    // 匹配驱动信息
    let mut macthInfo: Vec<(HwID, Vec<InfInfo>)> = Vec::new();

    // 提示：循环次数少的放在外层，减少内层变量的操作次数
    // 提示：一个设备信息 对应 多个匹配驱动信息

    // 遍历有问题的硬件id信息
    for idInfo in idInfo.iter() {
        // 空匹配信息
        let mut macthList: Vec<InfInfo> = Vec::new();

        // 优先对比硬件id
        // println!("正在匹配: {}", idInfo.Name);
        for haID in idInfo.HardwareIDs.iter() {
            // println!("     遍历中");
            // 遍历inf列表
            for InfInfo in infInfoList {
                // 遍历INF中的硬件id
                for infID in InfInfo.DriverList.iter() {
                    if haID.to_lowercase() == infID.to_lowercase() {
                        // println!("     匹配成功");
                        macthList.push(InfInfo.clone());
                    }
                }
            }
        }

        if !isAccurateMatch {
            // 对比兼容id
            for haID in idInfo.CompatibleIDs.iter() {
                // println!("     遍历中");
                // 遍历inf列表
                for InfInfo in infInfoList {
                    // 遍历INF中的硬件id
                    for infID in InfInfo.DriverList.iter() {
                        if haID.to_lowercase() == infID.to_lowercase() {
                            // println!("     匹配成功");
                            macthList.push(InfInfo.clone());
                        }
                    }
                }
            }
        }

        if macthList.len() == 0 {
            // 没有匹配到该设备的驱动信息，直接匹配下一个设备
            continue;
        }
        macthInfo.push((idInfo.clone(), macthList));
    }
    // println!("{:#?}", macthInfo);

    Ok(macthInfo)
}
