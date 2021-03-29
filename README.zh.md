# DriverIndexer

## 介绍

`DriverIndexer` 是用于创建、读取和安装驱动包索引的工具。

## 软件架构

使用`Rust`开发，调用`devcon.exe`获取硬件 id、安装驱动。

## 使用说明

本程序为命令行程序，故需要在其后面接参数运行，可通过`cmd`来运行。

### 创建驱动索引

`DriverIndexer.exe create-index 驱动路径 索引文件保存路径`

- `DriverIndexer.exe create-index D:\netcard index.json`
- `DriverIndexer.exe create-index D:\netcard D:\index.json`

### 加载驱动

- 无驱动索引: `DriverIndexer.exe load-driver 驱动路径/驱动包路径`
  - `DriverIndexer.exe load-driver D:\netcard`
  - `DriverIndexer.exe load-driver D:\netcard.7z`
- 有驱动索引: `DriverIndexer.exe load-driver 驱动包路径 驱动路径`
  - `DriverIndexer.exe load-driver D:\netcard.7z netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard.7z D:\netcard.json`

### 整理驱动

`DriverIndexer.exe classify-driver 驱动路径`

- `DriverIndexer.exe classify-driver D:\netcard`

### 开启日志

`DriverIndexer.exe 命令 参数 --debug`

- `DriverIndexer.exe create-index D:\netcard index.json --debug`
- `DriverIndexer.exe load-driver D:\netcard --debug`

### 查看命令帮助

`DriverIndexer.exe 命令名 --help`

- `DriverIndexer.exe load-driver --help`
- `DriverIndexer.exe create-index --help`

## 开源许可

`DriverIndexer` 使用 GPL V3.0 协议开源，请尽量遵守开源协议。

## 致谢

- Hydrogen
- Lightning
- Skyfree
- 红毛樱木

## 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request
