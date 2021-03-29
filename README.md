# DriverIndexer

## Introduction

`DriverIndexer` is a tool for creating, reading and installing driver package indexes.

## Software Architecture

Use `Rust` to develop, call `devcon.exe` to obtain the hardware id and install the driver.

## Instructions for use

This program is a command line program, so it needs to be run with parameters after it, and it can be run through `cmd`.

### Create Drive Index

`DriverIndexer.exe create-index Drive path Index file save path`

- `DriverIndexer.exe create-index D:\netcard index.json`
- `DriverIndexer.exe create-index D:\netcard D:\index.json`

### Load the driver

- No driver index: `DriverIndexer.exe load-driver drive path/drive package path`
  - `DriverIndexer.exe load-driver D:\netcard`
  - `DriverIndexer.exe load-driver D:\netcard.7z`
- Drive index: `DriverIndexer.exe load-driver drive package path drive path`
  - `DriverIndexer.exe load-driver D:\netcard.7z netcard.json`
  - `DriverIndexer.exe load-driver D:\netcard.7z D:\netcard.json`

### Organize the drive

`DriverIndexer.exe classify-driver drive path`

- `DriverIndexer.exe classify-driver D:\netcard`

### Open log

`DriverIndexer.exe command parameter --debug`

- `DriverIndexer.exe create-index D:\netcard index.json --debug`
- `DriverIndexer.exe load-driver D:\netcard --debug`

### View command help

`DriverIndexer.exe command name --help`

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
