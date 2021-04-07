# DriverIndexer

[简体中文](README.zh.md) [English](README.md)

## Introduction

`DriverIndexer` is a tool for creating, reading and installing driver package indexes.

### What is the use of `DriverIndexer`?

For the convenience of installation, many people pack multiple drivers into one driver package. Generally, installing the driver package needs to decompress all of them, and then call tools such as `Dpinst` to install the driver. This method is very time-consuming and performance-consuming. The function of `DriverIndexer` is to decompress the currently matched driver on demand and install it automatically.

### What is an index file?

Since the hardware ID is stored in the INF file, decompression on demand needs to establish a correspondence between the hardware ID list in the INF file and the path of the driver in the driver package. This correspondence is called an index. According to the index, the driver that needs to be installed can be decompressed, and then installed.

### Why does the index file use the `JSON` format?

Under normal circumstances, the index in a driver package will not exceed 10MB, and the amount of data of this size is sufficient to use the `JSON` format.

### Why can I install the driver without specifying the index file?

When no index file is specified, `DriverIndexer` will decompress all INF files in the driver package, create an index instantly, and finally match the driver according to the index information.

### Why use `Devcon` to install the driver?

It has been tested to install the driver `Devcon` faster than `Dpinst`, `Pnputi`, etc., and `Dpinst` currently no longer provides updates from Microsoft.

### What is the difference with IT Sky Universal Drive/Driver President?

`DriverIndexer` is a command line program, which means that the driver can be installed silently without interface interaction, making the experience the same as the built-in driver.

### Where can I get the driver package?

We also advocate collecting the driver package by ourselves, and if necessary, you can also extract the driver package in the current driver software by yourself (this type of driver package is copyright-free)

## Software Architecture

Use `Rust` to develop, call `Devcon.exe` to obtain the hardware id and install the driver.

## Instructions for use

This program is a command line program, so it needs to be run with parameters after it, and it can be run through `cmd`. Note: Please run `cmd` as an administrator.

### Create Index

`DriverIndexer.exe create-index DrivePath IndexFileSavePath`

- Create index from file
    -`DriverIndexer.exe create-index D:\netcard.7z index.json`
    -`DriverIndexer.exe create-index D:\netcard.7z D:\index.json`
- Create an index from the catalog
    -`DriverIndexer.exe create-index D:\netcard index.json`
    -`DriverIndexer.exe create-index D:\netcard D:\index.json`

### Load the driver

- No driver index: `DriverIndexer.exe load-driver drivePath/drivePackagePath`
  - `DriverIndexer.exe load-driver D:\netcard`
  - `DriverIndexer.exe load-driver D:\netcard.7z`
- Drive index: `DriverIndexer.exe load-driver drivePackagePath indexPath`
  - `DriverIndexer.exe load-driver D:\netcard.7z netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard.7z D:\netcard.json`
- Specify drive type: `DriverIndexer.exe load-driver drivePackagePath --DriveClass DriveType`
  - `DriverIndexer.exe load-driver D:\netcard.7z --DriveClass Net`
  - `DriverIndexer.exe load-driver D:\netcard.7z --DriveClass Display`

### Organize the drive

`DriverIndexer.exe classify-driver drivePath`

- `DriverIndexer.exe classify-driver D:\netcard`

### Open log

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

## Participate in Contribution

1. Fork this warehouse
2. Create new Feat_xxx branch
3. Submit the code
4. New Pull Request
