#[cfg(test)]
mod Test {
    use std::path::{PathBuf};
    use crate::cli::isValidPathIncludeWildcard;
    use crate::utils::devcon::Devcon;
    use std::fs::File;
    use crate::utils::util::compareVersiopn;
    use std::error::Error;
    use crate::i18n::getLocaleText;

    // 文件解压测试
    #[test]
    fn unzipTest() {
        use crate::utils::sevenZip::Zip7z;

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\USB无线网卡驱动.zip");
        let outPath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\outPath");

        let zip = Zip7z::new().unwrap();
        println!("{:?}", zip.extractFilesFromPath(&basePath, "", &outPath));
    }

    // 文件遍历测试
    #[test]
    fn fileListTest() {
        use crate::utils::util::getFileList;

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\新建文件夹");

        let fileList = getFileList(&basePath, "*.inf").unwrap();
        println!("{:?}", fileList);
        println!("{:?}", fileList.len());
    }

    // 多国语言支持
    #[test]
    fn Language() {
        unsafe {
            let langID = winapi::um::winnls::GetUserDefaultUILanguage();
            // 2052为简体中文
            println!("{:?}", langID);
        }

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
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop");
        let infPath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\SN9C128.inf");
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
        use crate::subCommand::create_index::getMatchInfo;

        Devcon::new().unwrap().removeDevice(r"USB\VID_0BDA&PID_B711&REV_0200").unwrap();

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动");
        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network");
        let infList = getFileList(&basePath, "*.inf").unwrap();

        let mut infInfoList: Vec<InfInfo> = Vec::new();
        for item in infList.iter() {
            infInfoList.push(InfInfo::parsingInfFile(&basePath, item).unwrap());
        }

        // infInfoList.sort_by(|b, a| compareVersiopn(&*a.Version, &*b.Version));

        // let mut versionList: Vec<String> = Vec::new();
        // for item in infInfoList {
        //     println!("{:?}", item);
        // versionList.push(item.Version);
        // }
        // versionList.sort_by(|a, b| compareVersiopn(a, b));
        // println!("{:?}", versionList);

        println!("{:#?}", getMatchInfo(&infInfoList, None));
    }

    // 版本号对比测试
    #[test]
    fn versionMatches() {
        println!("{:?}", compareVersiopn("1.0", "2.0"));
    }

    // 驱动加载测试
    #[test]
    fn loadDriverTest() {
        use crate::utils::devcon::Devcon;
        use crate::subCommand::load_driver::loadDriver;

        Devcon::new().unwrap().removeDevice(r"USB\VID_0BDA&PID_B711&REV_0200").unwrap();

        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\Network.zip");
        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动.zip");
        // let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network");

        let index = None;
        // let index = Some(PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\Network\USB无线网卡驱动.json"));
        loadDriver(&basePath, index, None);
    }

    // 驱动整理测试
    #[test]
    fn classifyDriverTest() {
        use crate::subCommand::classify_driver::classify_driver;

        let basePath = PathBuf::from(r"C:\Users\Administrator.W10-20201229857\Desktop\万能网卡驱动-驱动精灵");
        classify_driver(&basePath);
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

    // 异步多线程测试
    #[test]
    fn asyncTest() {
        // use tokio::task;
        //
        // task::spawn(task());

        println!("==============");
    }

    async fn task() {
        println!("aaa");
    }
}
