# DriverIndexer

[简体中文](README.zh.md) [English](README.md)

## 介绍

`DriverIndexer` 是用于创建、读取和安装驱动包索引的工具。

### `DriverIndexer`有什么用？

很多人为了安装方便，将多个驱动打包为一个驱动包，而一般安装驱动包需要全部解压，再调用`Dpinst`等工具进行安装驱动，这种方法非常消耗时间与性能。`DriverIndexer`的功能就是按需解压当前匹配的驱动，并自动安装。

### 索引文件是什么？

由于硬件ID存储在INF文件内，按需解压需要建立 `INF文件中硬件ID列表` 与 `驱动包内驱动所在路径` 的对应关系，这一对应关系我们称之为索引。根据索引就能解压当前需要安装的驱动，然后再安装。

### 为什么索引文件使用`JSON`格式？

通常情况下，一个驱动包内的索引不会超过10MB，而这个大小的数据量使用`JSON`格式足够了。

### 为什么可以不指定索引文件来安装驱动？

当不指定索引文件时，`DriverIndexer`会解压驱动包中的所有INF文件，即时建立索引，最后根据索引的信息来匹配驱动。

### 为什么使用`Devcon`安装驱动？

经测试安装驱动`Devcon`比`Dpinst`、`Pnputi`等速度要快，且`Dpinst`目前微软已不再提供更新。

### 与 IT天空万能驱动/驱动总裁 有何区别？

`DriverIndexer`是命令行程序，这意味着可以静默安装驱动，不需要进行界面交互，使得体验与内置驱动一样。

### 从哪里获取驱动包？

> 我们更提倡自己下载、搜集驱动包，如有需求也可自行提取目前各个驱动软件内的驱动包（一般此类驱动包无版权）

以下为推荐的驱动包下载网站（均免费、无加密）

- [DriverPack](https://drp.su/en/foradmin)
- [3DP](https://www.3dpchip.com/3dpchip/3dp/net_down.php)
- [DriverOff](https://driveroff.net/category/dp)
- [BatPEDriver](http://forum.ru-board.com/topic.cgi?forum=62&topic=24098&start=71&limit=1&m=1#1)

## 软件架构

使用`Rust`开发，调用`Devcon.exe`获取硬件 id、安装驱动。

### 驱动匹配规则

1. 仅匹配**未安装驱动**的设备
2. 专用驱动优先级大于公版
3. 高版本优先级大于低版本

## 使用说明

本程序为命令行程序，故需要在其后面接参数运行，可通过`cmd`来运行，注意：请使用管理员身份运行`cmd`。

### 创建索引

`DriverIndexer.exe create-index 驱动路径 索引文件保存路径`

- 从文件中创建索引
    - `DriverIndexer.exe create-index D:\netcard.7z index.json`
    - `DriverIndexer.exe create-index D:\netcard.7z D:\index.json`
- 从目录中创建索引
    - `DriverIndexer.exe create-index D:\netcard index.json`
    - `DriverIndexer.exe create-index D:\netcard D:\index.json`

### 加载驱动

- 无驱动索引: `DriverIndexer.exe load-driver 驱动路径/驱动包路径`
  - `DriverIndexer.exe load-driver D:\netcard`
  - `DriverIndexer.exe load-driver D:\netcard.7z`
- 有驱动索引: `DriverIndexer.exe load-driver 驱动包路径 驱动路径`
  - `DriverIndexer.exe load-driver D:\netcard.7z netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard.7z D:\netcard.json`
- 指定驱动类型：`DriverIndexer.exe load-driver 驱动包路径 --DriveClass 驱动类型`
  - `DriverIndexer.exe load-driver D:\AllDriver.7z --DriveClass Net`
  - `DriverIndexer.exe load-driver D:\AllDriver.7z --DriveClass Display`

### 整理驱动

`DriverIndexer.exe classify-driver 驱动路径`

- `DriverIndexer.exe classify-driver D:\netcard`

### 开启日志

`DriverIndexer.exe 命令 参数 --debug`

- `DriverIndexer.exe create-index D:\netcard index.json --debug`
- `DriverIndexer.exe load-driver D:\netcard --debug`

### 查看帮助

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
