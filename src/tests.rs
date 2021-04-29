#![allow(unused_imports)]

mod Tests {
    use std::path::{PathBuf};
    use crate::cli::isValidPathIncludeWildcard;
    use crate::utils::devcon::Devcon;
    use crate::utils::util::compareVersiopn;
    use std::{thread, mem};
    use std::time::{SystemTime, UNIX_EPOCH};

    // 文件解压测试
    #[test]
    fn unzipTest() {
        use crate::utils::sevenZip::sevenZip;

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\USB无线网卡驱动.zip");
        let outPath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\outPath");

        let zip = sevenZip::new().unwrap();
        println!("{:?}", zip.extractFilesFromPath(&basePath, "", &outPath));
    }

    // 文件遍历测试
    #[test]
    fn fileListTest() {
        use crate::utils::util::getFileList;

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network");

        let fileList = getFileList(&basePath, "*.inf").unwrap();
        println!("{:?}", fileList);
        println!("{:?}", fileList.len());
    }

    // 多国语言支持
    #[test]
    fn Language() {
        use crate::i18n::getLocaleText;
        use crate::bindings::Windows;

        unsafe {
            // let langID = winapi::um::winnls::GetUserDefaultUILanguage();
            let langID = Windows::Win32::Intl::GetUserDefaultUILanguage();

            // 2052为简体中文
            println!("{:?}", langID);
        }
        return;

        use unic_langid::{LanguageIdentifier, langid};
        use fluent_templates::{Loader, static_loader};

        const US_ENGLISH: LanguageIdentifier = langid!("en-US");
        const ZH_CHINEXE: LanguageIdentifier = langid!("zh-CN");

        static_loader! {
            static LOCALES = {
                locales: "./src/i18n",
                fallback_language: "en-US",
            };
        }

        assert_eq!("Hello World!", LOCALES.lookup(&US_ENGLISH, "hello-world"));
        assert_eq!("你好，世界!", LOCALES.lookup(&ZH_CHINEXE, "hello-world"));

        println!("{}", LOCALES.lookup(&ZH_CHINEXE, "hello-world"));

        // println!("{}", getLocaleText("hello-world"));
        // println!("{}", getLocaleText("greeting", Some(hash_map!("name".to_string() => "Alice".into()))));
    }

    // INF解析测试
    #[test]
    fn parsingInfFileTest() {
        use crate::subCommand::create_index::InfInfo;

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\Net");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动");
        let infPath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动\netrtl8188gu.inf");

        println!("{:#?}", InfInfo::parsingInfFile(&basePath, &infPath).unwrap());
    }

    // 正则表达式测试
    #[test]
    fn reTest() {
        use regex::RegexSet;
        use regex::RegexSetBuilder;

        let reSet = RegexSetBuilder::new(&["USB", "45646"])
            .case_insensitive(true)
            .build().unwrap();
        let aaa = reSet.matches("USB SADFASDF SDAFFDAS 45646");

        // let bbb mut = aaa.into_iter();
        // println!("{:?}", bbb.next());

        // for item in aaa.into_iter() {
        // println!("{:?}", aaa.matched(item));
        // println!("{:?}", item.next());
        // }
    }

    // 驱动匹配测试
    #[test]
    fn matchDriverTest() {
        use crate::utils::util::getFileList;
        use crate::subCommand::create_index::InfInfo;
        use crate::subCommand::load_driver::getMatchInfo;

        Devcon::new().unwrap().removeDevice(r"USB\VID_0BDA&PID_B711&REV_0200").unwrap();

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network");

        let infList = getFileList(&basePath, "*.inf").unwrap();
        let infInfoList = InfInfo::parsingInfFileList(&basePath, &infList);

        let devcon = Devcon::new().unwrap();

        // 扫描以发现新的硬件
        devcon.rescan().unwrap();
        // 获取真实硬件id信息
        let hwIdList = devcon.getRealIdInfo(None).unwrap();
        // 获取有问题的硬件id信息
        // let hwIdList = devcon.getProblemIdInfo(hwIdList).unwrap();
        // println!("{:#?}", hwIdList);

        // 匹配硬件id
        let time1 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let matchInfo = getMatchInfo(hwIdList, &infInfoList).unwrap();
        let time2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        println!("{:#?}", matchInfo);
        println!("匹配耗时：{:?}", time2 - time1);
    }

    // 驱动加载测试
    #[test]
    fn loadDriverTest() {
        use crate::utils::devcon::Devcon;
        use crate::subCommand::load_driver::loadDriver;

        Devcon::new().unwrap().removeDevice(r"USB\VID_0BDA&PID_B711&REV_0200").unwrap();

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动.zip");

        let index = None;
        // let index = Some(PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动.json"));

        loadDriver(&basePath, index, false, None);
    }

    // 驱动整理测试
    #[test]
    fn classifyDriverTest() {
        use crate::subCommand::classify_driver::classify_driver;

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\万能网卡驱动-驱动精灵");
        classify_driver(&basePath);
    }

    // 版本号对比测试
    #[test]
    fn versionMatches() {
        println!("{:?}", compareVersiopn("1.0", "2.0"));
    }

    // 编码测试
    #[test]
    fn encodingTest() {
        use std::fs::File;
        use chardet::{detect, charset2encoding};
        use encoding::label::encoding_from_whatwg_label;
        use encoding::DecoderTrap;
        use std::io::Read;

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop");
        let infPath = basePath.join(r"Network\Wlan\Atheros\OEM\\Dell\athw10x.inf");
        // let infPath = basePath.join(r"Network\Lan\Realtek\Mod\20180105\netrtwlane.inf");

        // println!("{}", InfInfo::parsingInfFile(&basePath, &infPath).unwrap().driverList.len());

        // 读取inf文件（使用UFT-16）
        let mut file = File::open(&infPath).unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf).unwrap();

        let result = detect(&fileBuf);
        let coder = encoding_from_whatwg_label(charset2encoding(&result.0)).unwrap();
        let utf8reader = coder.decode(&fileBuf, DecoderTrap::Ignore).expect("Error");
        println!("{:?}", utf8reader);

        // let infContent = UTF_16LE.decode(&*fileBuf, DecoderTrap::Strict).unwrap();
        // println!("=================");
        // println!("{:?}", infContent);
    }

    // 通配符支持
    #[test]
    fn wildcard() {
        use crate::utils::util::getFileList;

        let path = PathBuf::from(r"D:\Project\FirPE\WinPE插件");
        let fileName = path.file_name().unwrap().to_str().unwrap();
        if fileName.contains("*") || fileName.contains("?") {
            println!("{}", getFileList(&PathBuf::from(&path.parent().unwrap()), fileName).unwrap().len());
        }
        for item in getFileList(&path, "*.7z").unwrap() {
            println!("{}", item.to_str().unwrap());
        }
    }

    #[test]
    fn wildcard2() {
        println!("{:?}", isValidPathIncludeWildcard(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\aaa.zip".to_string()));
    }

    // 环境变量测试
    #[test]
    fn EnvTest() {
        use std::env;

        for (key, value) in env::vars() {
            println!("  {}  =>  {}", key, value);
        }
    }

    // 多线程测试
    #[test]
    fn multithreadingTest() {
        use std::thread;

        // 模拟有20个元素
        let mut list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20].iter();

        let t1 = thread::spawn(move || {
            for _n in 0..2 {
                let value = list.next();
                if value.is_some() {
                    println!("{}", value.unwrap());
                }
            };
            list
        });

        t1.join().unwrap();
    }

    // 系统位宽判断测试
    #[test]
    fn getSystemBitWidthTest() {
        match std::mem::size_of::<&char>() {
            4 => println!("x32"),
            8 => println!("x64"),
            _ => {}
        }
    }

    // SetupAPI测试
    #[test]
    fn setupAPITest() {
        use crate::utils::setupAPI;

        // unsafe {
        //     let ClassArray = EnumDeviceClasses();
        //     for item in ClassArray {
        //         EnumDevices(item);
        //     }
        // }
        unsafe {
            setupAPI::getDeviceInfo();
        }
    }

    // newdevAPI测试
    #[test]
    fn newdevAPITest() {
        use crate::utils::newdevAPI::updateDriverForPlugAndPlayDevices;

        unsafe {
            let result = updateDriverForPlugAndPlayDevices(&PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动\netrtl8188gu.inf"), r"USB\VID_0BDA&PID_B711".to_string());
            println!("{:?}", result);
        }
    }

    #[test]
    fn writeRegTest() {
        use winreg::RegKey;
        use winreg::enums::HKEY_LOCAL_MACHINE;

        // 关闭驱动数字验证
        // HKLM\SYSTEM\Setup\SystemSetupInProgress=#1

        // 恢复驱动数字验证（默认）
        // HKLM\SYSTEM\Setup\SystemSetupInProgress=#0

        let setup = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SYSTEM\Setup").unwrap();

        setup.set_value("SystemSetupInProgress", &(1 as u32)).unwrap();

        let value: u32 = setup.get_value("SystemSetupInProgress").unwrap();
        println!("{:?}", value);
    }
}
