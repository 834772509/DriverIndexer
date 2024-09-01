# DriverIndexer

[简体中文](README.zh.md) [English](README.md)

## Introduction

`DriverIndexer` is a tool for creating, reading and installing driver package indexes.

### What is the use of `DriverIndexer`?

For the convenience of installation, many people pack multiple drivers into one driver package. Generally, installing the driver package needs to decompress all of them, and then call tools such as `Dpinst` to install the driver. This method is very time-consuming and performance-consuming. The function of `DriverIndexer` is to decompress the currently matched driver on demand and install it automatically.

### What is an index file?

Since the hardware ID is stored in the INF file, decompression on demand needs to establish a correspondence between the hardware ID list in the INF file and the path of the driver in the driver package. This correspondence is called an index. According to the index, the driver that needs to be installed can be decompressed, and then installed.

### Why does the index file use the `JSON` format?

Under normal circumstances, the index in a driver package will not exceed 10MB, and this size of data is enough to use the common `JSON` format.

### Why can I install the driver without specifying the index file?

When no index file is specified, `DriverIndexer` will decompress all INF files in the driver package, create an index instantly, and finally match the driver according to the index information.

### What is the difference with EasyDrv/DrvCeo?

`DriverIndexer` is a command line program, which means that the driver can be installed silently without interface interaction, making the experience the same as the built-in driver.

### Where can I get the driver package?

> We recommend downloading and collecting driver packages by ourselves. If necessary, you can also extract the driver packages in each driver software by yourself (generally, such driver packages are copyright-free)

The following are the recommended downloading websites for the driver package (all free and without encryption)

- [DriverPack](https://drp.su/en/foradmin?_blank)
- [3DP](https://www.3dpchip.com/3dpchip/3dp/net_down.php?_blank)
- [DriverOff](https://driveroff.net/category/dp?_blank)
- [BatPEDriver](http://forum.ru-board.com/topic.cgi?forum=62&topic=24098&start=71&limit=1&m=1#1?_blank)

## Software Architecture

Use `Rust` to write, call `Devcon.exe` to obtain hardware information, and use API to install device drivers.

### Drive matching rules

1. By default, it only matches devices with no driver installed
2. The priority of the dedicated driver is greater than that of the public version
3. The higher version has priority over the lower version
4. Three matches (to prevent unsuccessful installation of some drivers)

## Instructions for use

This program is a command line program, so it needs to be run with parameters after it. For example, double-clicking the program directly will cause a "flash back" phenomenon. You can run it through terminals such as `cmd` and `PowerShell`.  
Note: Please run the terminal as an **administrator**.

### Create Index

`DriverIndexer.exe create-index DrivePath IndexFileSavePath [-p UnzipPassword]`

- Create index from file
    -`DriverIndexer.exe create-index D:\netcard.7z index.json`
    -`DriverIndexer.exe create-index D:\netcard.7z D:\index.json`
- Create an index from the catalog
    -`DriverIndexer.exe create-index D:\netcard index.json`
    -`DriverIndexer.exe create-index D:\netcard D:\index.json`

### Load the driver

`DriverIndexer.exe load-driver drivePath/drivePackagePath [-p UnzipPassword] [--AllDevice] [--ExtractDriver] [--DriveClass DriveClass]`

- No driver index: `DriverIndexer.exe load-driver drivePath/drivePackagePath`
  - `DriverIndexer.exe load-driver D:\netcard`
  - `DriverIndexer.exe load-driver D:\netcard.7z`
  - `DriverIndexer.exe load-driver D:\netcard\*.7z`
- Drive index: `DriverIndexer.exe load-driver drivePath/drivePackagePath indexPath`
  - `DriverIndexer.exe load-driver D:\netcard.7z netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard.7z D:\netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard\*.7z D:\netcard\*.json`
- Specify drive type: `DriverIndexer.exe load-driver drivePath/drivePackagePath --DriveClass DriveType`
  - `DriverIndexer.exe load-driver D:\AllDriver.7z --DriveClass Net`
  - `DriverIndexer.exe load-driver D:\AllDriver.7z --DriveClass Display`
- Match all devices：`DriverIndexer.exe load-driver drivePath/drivePackagePath --AllDevice`
  - `DriverIndexer.exe load-driver D:\netcard.7z --AllDevice`
- Decompress driver only：`DriverIndexer.exe load-driver drivePath/drivePackagePath --ExtractDriver UnzipDirectory`
  - `DriverIndexer.exe load-driver D:\netcard.7z --ExtractDriver D:\netcard`

### Organize the drive

`DriverIndexer.exe classify-driver drivePath`

- `DriverIndexer.exe classify-driver D:\netcard`

### Create driver package program

> The driver package program merges `DriverIndexer` with the driver package to generate an exe binary executable file. The generated executable file will automatically read its own driver package and only decompress the required driver (avoid secondary decompression).

Note: **You cannot set a password for the driver package**.

`DriverIndexer.exe create-driver driver path output path`

- Create a program driver package from a file
  - `DriverIndexer.exe create-driver D:\netcard.7z D:\netcard.exe`
- Create a program driver package from a directory
  - `DriverIndexer.exe create-driver D:\netcard D:\netcard.exe`

### Open log

The logs will be stored in the program directory: `\DriverIndexer.log`

`DriverIndexer.exe commandParameter --debug`

- `DriverIndexer.exe create-index D:\netcard index.json --debug`
- `DriverIndexer.exe load-driver D:\netcard --debug`

### View help

`DriverIndexer.exe commandName --help`

- `DriverIndexer.exe load-driver --help`
- `DriverIndexer.exe create-index --help`

## Open source license

`DriverIndexer` uses GPL V3.0 agreement to open source, please try to abide by the open source agreement.

## Thanks

- Hydrogen
- Lightning
- Skyfree
- Red Sakuragi
- Little duck
- Gross Profit

## Participate in Contribution

1. Fork this warehouse
2. Create new Feat_xxx branch
3. Submit the code
4. New Pull Request
