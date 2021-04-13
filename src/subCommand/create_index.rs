use std::path::PathBuf;
use std::error::Error;
use std::fs;
use encoding::{DecoderTrap};
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::Read;
use crate::utils::util::{getFileList, getStringCenter, compareVersiopn};
use crate::utils::console::{writeConsole, ConsoleType};
use serde::{Serialize, Deserialize};
use chardet::{detect, charset2encoding};
use encoding::label::encoding_from_whatwg_label;
use crate::utils::devcon::{HwID, Devcon};
use crate::TEMP_PATH;
use crate::utils::sevenZip::Zip7z;
use crate::i18n::getLocaleText;
use std::collections::HashMap;
use fluent_templates::fluent_bundle::FluentValue;

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
    /// 驱动日期
    pub(crate) Date: String,
    /// 驱动版本
    pub(crate) Version: String,
    /// 驱动硬件id列表
    pub(crate) DriverList: Vec<String>,
}

impl InfInfo {
    /// 解析inf文件
    /// 参数1: inf 基本路径（父路径）
    /// 参数2: inf 文件路径
    pub fn parsingInfFile(basePath: &PathBuf, infFile: &PathBuf) -> Result<InfInfo, Box<dyn Error>> {
        // let regExpression = [r"PCI\\.*?&.*?&[^; \f\n\r\t\v]+", r"USB\\.*?&[^; \f\n\r\t\v]+", ];
        lazy_static! {
            // 所有类别取自 [HKLM\SYSTEM\ControlSet001\Enum]
            static ref DRIVERTYPE: [&'static str; 22] = ["ACPI", "ACPI_HAL", "BTH", "BTHENUM", "BTHHFENUM", "DISPLAY", "HDAUDIO", "HID", "HTREE", "PCI", "ROOT", "SCSI", "SD", "STORAGE", "SW", "SWD", "TERMINPUT_BUS", "TS_USB_HUB_Enumerator", "UEFI", "UMB", "USB", "USBSTOR"];
            // 驱动ID匹配正则表达式
            static ref REGEXPRESSION: Vec<String> = DRIVERTYPE.iter().map(|&item| format!(r",{}\\[^,; \f\n\r\t\v]+", item)).collect();
            // 编译正则表达式（确保正则表达式只被编译一次）
            static ref REGEXPRESSIONLIST: Vec<Regex> = REGEXPRESSION.iter().map(|item| RegexBuilder::new(item).case_insensitive(true).build().unwrap()).collect();
        }

        // 读取INF文件
        let mut file = File::open(&infFile)?;
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf)?;

        // 自动识别编码并以UTF-8读取
        let result = detect(&fileBuf);
        let coder = encoding_from_whatwg_label(charset2encoding(&result.0)).unwrap();
        let infContent = coder.decode(&fileBuf, DecoderTrap::Ignore)?;

        // 去除INF内的所有 空格 与 tab符
        let infContent = infContent.replace(" ", "").replace("	", "");

        let mut idList: Vec<String> = Vec::new();

        // 遍历正则表达式
        for re in REGEXPRESSIONLIST.iter() {
            // 匹配正则表达式
            let hwIdList: Vec<_> = re.find_iter(&*infContent).collect();
            for id in hwIdList {
                // 删除逗号、转为大写
                let id = id.as_str().replace(",", "").to_uppercase();
                // 检测是否重复
                if !idList.contains(&id) { idList.push(id); }
            }
        }

        // 闭包函数-取INF配置项内容
        let getInfItemFun = |itemName: &str| {
            if let Ok(re) = Regex::new(&*format!(r"{}=\S*", itemName)) {
                let contentList: Vec<_> = re.find_iter(&*infContent).collect();
                for item in contentList {
                    let resultContent = item.as_str().replace(&*format!(r"{}=", itemName), "");
                    return resultContent;
                }
            }
            return "".to_string();
        };

        // 获取INF类别
        // let class = getStringCenter(&infContent, "Class=", "\n").unwrap_or("".to_string()).replace("\r", "");
        let class = getInfItemFun("Class");

        // 获取INF版本、日期
        let dateAndVersion = getInfItemFun("DriverVer");
        let dateAndVersion: Vec<&str> = dateAndVersion.split(",").collect();
        let date = *dateAndVersion.get(0).unwrap_or(&"");
        let version = *dateAndVersion.get(1).unwrap_or(&"");

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

        // 获取INF文件相对路径
        let parentPath = infFile.parent().unwrap().strip_prefix(basePath)?;

        Ok(InfInfo {
            Path: parentPath.to_str().unwrap().parse().unwrap(),
            Inf: infFile.file_name().unwrap().to_str().unwrap().parse().unwrap(),
            Class: class,
            Provider: provider,
            Date: date.to_string(),
            Version: version.to_string(),
            DriverList: idList,
        })
    }
}

pub fn createIndex(basePath: &PathBuf, saveIndexPath: &PathBuf) {
    writeConsole(ConsoleType::Info, &*getLocaleText("Processing", None));

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

    let (mut successCount, mut ErrorCount, mut blankCount) = (0, 0, 0);

    // 遍历INF文件
    for item in infList.iter() {
        let arg = hash_map!("path".to_string() => item.to_str().unwrap().into());
        if let Ok(currentInfo) = InfInfo::parsingInfFile(&infParentPath, item) {
            if currentInfo.DriverList.len() == 0 {
                blankCount += 1;
                writeConsole(ConsoleType::Warning, &*getLocaleText("no-hardware", Some(arg)));
                continue;
            }
            successCount += 1;
            infInfoList.push(currentInfo);
        } else {
            ErrorCount += 1;
            writeConsole(ConsoleType::Err, &*getLocaleText("inf-parsing-err", Some(arg)));
        }
    };

    if let Err(_e) = saveDataFromJson(&infInfoList, &indexPath) {
        writeConsole(ConsoleType::Err, &*getLocaleText("index-save-failed", None));
        return;
    }
    let arg: HashMap<String, FluentValue> = hash_map!(
        "total".to_string() => infList.len().into(),
        "success".to_string() => successCount.to_string().into(),
        "error".to_string() => ErrorCount.to_string().into(),
        "blankCount".to_string() => blankCount.to_string().into(),
    );
    writeConsole(ConsoleType::Info, &*getLocaleText("total-info", Some(arg)));
    writeConsole(ConsoleType::Success, &*getLocaleText("saveInfo", Some(hash_map!("path".to_string() => indexPath.to_str().unwrap().into()))));
}

/// 保存inf数据（通过json）
fn saveDataFromJson(Data: &Vec<InfInfo>, savaPath: &PathBuf) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(&Data)?;
    fs::write(savaPath, json)?;
    Ok(())
}

/// 获取索引数据
/// 参数1: inf文件路径
pub fn parsingIndexData(indexPath: &PathBuf) -> Result<Vec<InfInfo>, Box<dyn Error>> {
    let mut indexFile = File::open(indexPath)?;
    let mut indexContent = String::new();
    indexFile.read_to_string(&mut indexContent)?;
    let json: Vec<InfInfo> = serde_json::from_str(&*indexContent)?;
    Ok(json)
}

/// 获取匹配驱动的信息
/// 参数1: INF驱动信息列表
/// 参数2: 是否为精准匹配（不匹配兼容ID）
pub fn getMatchInfo(infInfoList: &Vec<InfInfo>, driveClass: Option<&str>) -> Result<Vec<(HwID, Vec<InfInfo>)>, Box<dyn Error>> {
    let devcon = Devcon::new()?;
    // 扫描以发现新的硬件
    devcon.rescan()?;
    // 获取有问题的硬件id信息
    let idInfo = devcon.getProblemIdInfo()?;

    // 匹配驱动信息
    let mut macthInfo: Vec<(HwID, Vec<InfInfo>)> = Vec::new();

    // 提示：
    // 循环次数少的放在外层，减少内层变量的操作次数
    // 一个设备信息 对应 多个匹配驱动信息

    // 规则：
    // 1. 专用驱动优先级大于公版
    // 2. 高版本优先级大于低版本

    // 遍历有问题的硬件id信息
    for idInfo in idInfo.iter() {
        // 闭包函数-匹配
        let matchFn = |haID: &String| {
            let mut macthList: Vec<InfInfo> = Vec::new();
            // 遍历INF信息列表
            for infInfoItem in infInfoList.iter() {
                let mut matchInfInfo = InfInfo {
                    DriverList: vec![],
                    ..infInfoItem.clone()
                };
                // 遍历INF中的硬件id
                for infID in infInfoItem.DriverList.iter() {
                    if haID.to_lowercase() == infID.to_lowercase() {
                        // 如果指定了安装驱动类别 且 不符合则跳过此INF
                        if driveClass.is_some() && infInfoItem.Class.to_lowercase() != driveClass.unwrap().to_string().to_lowercase() { break; }
                        matchInfInfo.DriverList.push(infID.to_string());
                    }
                }
                if !matchInfInfo.DriverList.is_empty() { macthList.push(matchInfInfo.clone()); }
            }
            // 排序：高版本优先级大于低版本
            macthList.sort_by(|b, a| compareVersiopn(&*a.Version, &*b.Version));
            return macthList;
        };

        // 创建匹配信息列表
        let mut macthList: Vec<InfInfo> = Vec::new();

        // 优先对比硬件id
        for haID in idInfo.HardwareIDs.iter() { macthList.append(&mut matchFn(haID)); }

        // 对比兼容id
        for haID in idInfo.CompatibleIDs.iter() { macthList.append(&mut matchFn(haID)); }

        // 没有匹配到该设备的驱动信息，则匹配下一个设备
        if macthList.len() == 0 { continue; }

        macthInfo.push((idInfo.clone(), macthList));
    }
    Ok(macthInfo)
}
