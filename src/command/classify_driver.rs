use std::error::Error;
use std::fs;
use std::path::Path;
use crate::utils::util::getFileList;


pub fn classify_driver(driverPath: &Path) -> Result<(), Box<dyn Error>> {
    // 遍历INF文件
    let infList = getFileList(driverPath, "*.inf").unwrap();
    for infFile in infList.iter() {
        // 将驱动目录重命名为INF文件名
        let newName = &infFile
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(infFile.file_stem().unwrap());
        fs::rename(infFile.parent().unwrap(), newName).unwrap();
    }

    // 重新遍历INF文件
    // let infList = getFileList(driverPath, "*.inf").unwrap();
    // for infFile in infList.iter() {
    //     let infInfo = InfInfo::parsingInfFile(&driverPath, &infFile).unwrap();
    //     let vendorName = infInfo.Provider;
    //     // 分类路径
    //     let classPath = driverPath.join(&vendorName);
    //     if !classPath.exists() { fs::create_dir(&classPath); }
    //
    //     Command::new("cmd")
    //         .arg("/c")
    //         .arg("move")
    //         .arg(infFile.parent().unwrap())
    //         .arg(&classPath)
    //         .output().unwrap();
    // }
    Ok(())
}
