use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::{fs, thread};
use crate::i18n::getLocaleText;
use crate::utils::console::{writeConsole, ConsoleType};
use crate::utils::sevenZIP::sevenZip;
use crate::utils::util::getFileList;
use crate::TEMP_PATH;
use chardet::{charset2encoding, detect};
use encoding::label::encoding_from_whatwg_label;
use encoding::DecoderTrap;
use fluent_templates::fluent_bundle::FluentValue;
use regex::{Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use serde::{Deserialize, Serialize};


/// INF驱动信息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InfInfo {
    /// 驱动路径
    pub(crate) Path: String,
    /// 驱动INF文件名
    pub(crate) Inf: String,
    /// 驱动类别
    pub(crate) Class: String,
    /// 驱动位宽
    pub(crate) Arch: Vec<String>,
    /// 驱动厂商
    // pub(crate) Provider: String,
    /// 驱动日期
    pub(crate) Date: String,
    /// 驱动版本
    pub(crate) Version: String,
    /// 驱动硬件id列表
    pub(crate) DriverList: Vec<String>,
}

impl InfInfo {
    /// 解析INF文件
    /// # 参数
    /// 1. inf 基本路径（父路径）
    /// 2. inf 文件路径
    pub fn parsingInfFile(basePath: &Path, infFile: &Path) -> Result<InfInfo, Box<dyn Error>> {
        // let regExpression = [r"PCI\\.*?&.*?&[^; \f\n\r\t\v]+", r"USB\\.*?&[^; \f\n\r\t\v]+", ];
        lazy_static! {
            // 所有类别取自 [HKLM\SYSTEM\ControlSet001\Enum]
            static ref DRIVERTYPE: [&'static str; 29] = ["1394","ACPI", "ACPI_HAL","AVC", "BTH", "BTHENUM", "BTHHFENUM", "DISPLAY", "HDAUDIO", "HID", "HTREE", "PCI","PCMCIA", "ROOT", "SCSI","SCMLD", "SD","SERENUM","Sensors", "STORAGE", "SW", "SWD", "TERMINPUT_BUS", "TS_USB_HUB_Enumerator", "UEFI", "UMB", "USB", "USBSTOR","VMBUS"];
            // 驱动ID匹配正则表达式
            static ref REGEXPRESSION: Vec<String> = DRIVERTYPE.iter().map(|&item| format!(r",{}\\[^,; \f\n\r\t\v]+", item)).collect();
            // 编译正则表达式（确保正则表达式只被编译一次）
            static ref REGEXPRESSIONLIST: Vec<Regex> = REGEXPRESSION.iter().map(|item| RegexBuilder::new(item).case_insensitive(true).build().unwrap()).collect();
            // 系统架构
            static ref SYSTEMARCH: [&'static str; 5] = ["NTx86", "NTia64", "NTarm", "NTarm64", "NTamd64"];
            static ref REGSYSTEMARCHIST: RegexSet = RegexSetBuilder::new(SYSTEMARCH.iter()).case_insensitive(true).build().unwrap();
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
        // let idList: HashSet<String>;

        // 遍历正则表达式
        for re in REGEXPRESSIONLIST.iter() {
            // 匹配正则表达式
            let hwIdList: Vec<_> = re.find_iter(&*infContent).collect();

            // let aaa = hwIdList.iter()
            //     // 删除逗号、转为大写
            //     .map(|id| id.as_str().replace(",", "").to_uppercase())
            //     .collect::<HashSet<String>>();
            //     // .collect::<Vec<String>>();
            for id in hwIdList.iter() {
                // 删除逗号、转为大写
                let id = id.as_str().replace(",", "").to_uppercase();
                // 检测是否重复
                if !idList.contains(&id) {
                    idList.push(id);
                }
            }
        }

        // 闭包函数-取INF配置项内容
        let getInfItemFun = |itemName: &str| {
            if let Ok(re) = Regex::new(&*format!(r"{}=[^ \t\n\r\f\v;]*", itemName)) {
                let contentList: Vec<_> = re.find_iter(&*infContent).collect();
                for item in contentList.iter() {
                    let resultContent = item.as_str().replace(&*format!(r"{}=", itemName), "");
                    return resultContent;
                }
            }
            "".to_string()
        };

        // 获取驱动类别
        let Class = getInfItemFun("Class");

        // 获驱动适用系统架构
        let Arch: Vec<String> = REGSYSTEMARCHIST
            .matches(&*infContent)
            .into_iter()
            .map(|index| SYSTEMARCH[index].to_string())
            .collect();

        // 获取驱动版本、日期
        let dateAndVersion = getInfItemFun("DriverVer");
        let dateAndVersion: Vec<&str> = dateAndVersion.split(',').collect();
        let Date = dateAndVersion.get(0).unwrap_or(&"").to_string();
        let Version = dateAndVersion.get(1).unwrap_or(&"").to_string();

        // 获取驱动厂商
        // let provider = infContent.getStringCenter("Provider=", "\n").unwrap_or("".to_string()).replace("\r", "").replace("%", "");
        // let provider = infContent.getStringCenter(&format!("{}=", provider), "\n").unwrap_or(provider).replace("\r", "").replace("\"", "")
        //     .replace("Corporation", "")
        //     .replace("SemiconductorCorp.", "")
        //     .replace("TechnologyCorp.", "")
        //     .replace(",Inc.", "")
        //     .replace("®", "")
        //     .replace("(R)", "");

        // 获取驱动文件相对路径
        let parentPath = infFile.parent().unwrap().strip_prefix(basePath)?;

        Ok(InfInfo {
            Path: parentPath.to_str().unwrap().parse().unwrap(),
            Inf: infFile
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .parse()
                .unwrap(),
            Class,
            Arch,
            // Provider: provider,
            Date,
            Version,
            DriverList: idList,
        })
    }

    /// 解析INF文件列表（多线程）
    /// 解析失败的INF将自动跳过
    /// # 参数
    /// 1. inf 基本路径（父路径）
    /// 2. inf 文件路径列表
    pub fn parsingInfFileList(basePath: &Path, infFileList: &[PathBuf]) -> Vec<InfInfo> {
        let (tx, rx) = mpsc::channel();
        for item in infFileList.iter() {
            let basePath = basePath.to_path_buf();
            let infFile = item.clone();
            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                if let Ok(infInfo) = InfInfo::parsingInfFile(&basePath, &infFile) {
                    tx1.send(infInfo).unwrap();
                }
            });
        }
        // 释放接受者，否则会卡住造成死锁
        drop(tx);
        // 接收数据并转为INF信息数组
        let infInfoList = rx.iter().collect::<Vec<InfInfo>>();
        infInfoList
    }

    /// 保存INF数据（通过JSON）
    /// #参数
    /// 1. INF列表
    /// 2. 索引文件保存路径
    pub fn saveIndexFromJson(Data: &[InfInfo], savaPath: &Path) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&Data)?;
        fs::write(savaPath, json)?;
        Ok(())
    }

    /// 解析索引数据
    /// # 参数
    /// 1. 索引文件路径
    pub fn parsingIndex(indexPath: &Path) -> Result<Vec<InfInfo>, Box<dyn Error>> {
        let mut indexFile = File::open(indexPath)?;
        let mut indexContent = String::new();
        indexFile.read_to_string(&mut indexContent)?;
        let json: Vec<InfInfo> = serde_json::from_str(&*indexContent)?;
        Ok(json)
    }
}

pub fn createIndex(basePath: &Path, saveIndexPath: &Path) {
    writeConsole(ConsoleType::Info, &*getLocaleText("processing", None));

    let zip = sevenZip::new().unwrap();

    // INF文件父路径
    let infParentPath;
    // INF文件列表
    let infList;
    // 保存索引路径
    let indexPath;

    if basePath.is_dir() {
        // 从驱动目录中创建索引文件
        infList = getFileList(&basePath, "*.inf").unwrap();
        infParentPath = basePath.to_path_buf();
        // 如果输入的索引路径是相对路径，则令实际路径为驱动目录所在路径
        indexPath = if saveIndexPath.is_relative() {
            basePath.join(&saveIndexPath)
        } else {
            saveIndexPath.to_path_buf()
        };
    } else {
        // 从文件中创建索引文件
        infParentPath = TEMP_PATH.join(basePath.file_stem().unwrap());
        // 解压INF文件
        zip.extractFilesFromPathRecurseSubdirectories(&basePath, "*.inf", &infParentPath)
            .unwrap();
        infList = getFileList(&infParentPath, "*.inf").unwrap();
        // 如果输入的索引路径是相对路径，则令实际实际为驱动包所在路径
        indexPath = if saveIndexPath.is_relative() {
            basePath.parent().unwrap().join(&saveIndexPath)
        } else {
            saveIndexPath.to_path_buf()
        };
    }

    let mut infInfoList: Vec<InfInfo> = Vec::new();

    let (mut successCount, mut ErrorCount, mut blankCount) = (0, 0, 0);

    // 遍历INF文件
    for item in infList.iter() {
        let arg = hash_map!("path".to_string() => item.to_str().unwrap().into());
        if let Ok(currentInfo) = InfInfo::parsingInfFile(&infParentPath, item) {
            if currentInfo.DriverList.is_empty() {
                blankCount += 1;
                writeConsole(
                    ConsoleType::Warning,
                    &*getLocaleText("no-hardware", Some(&arg)),
                );
                continue;
            }
            successCount += 1;
            infInfoList.push(currentInfo);
        } else {
            ErrorCount += 1;
            writeConsole(
                ConsoleType::Err,
                &*getLocaleText("inf-parsing-err", Some(&arg)),
            );
        }
    }

    if let Err(_e) = InfInfo::saveIndexFromJson(&infInfoList, &indexPath) {
        writeConsole(ConsoleType::Err, &*getLocaleText("index-save-failed", None));
        return;
    }
    let arg: HashMap<String, FluentValue> = hash_map!(
        "total".to_string() => infList.len().into(),
        "success".to_string() => successCount.to_string().into(),
        "error".to_string() => ErrorCount.to_string().into(),
        "blankCount".to_string() => blankCount.to_string().into(),
    );
    writeConsole(ConsoleType::Info, &*getLocaleText("total-info", Some(&arg)));
    let arg: HashMap<String, FluentValue> =
        hash_map!("path".to_string() => indexPath.to_str().unwrap().into());
    writeConsole(
        ConsoleType::Success,
        &*getLocaleText("saveInfo", Some(&arg)),
    );
}
