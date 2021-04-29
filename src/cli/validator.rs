use std::path::PathBuf;

/// 是否为有效的路径
pub fn isValidPath(path: String) -> Result<(), String> {
    if PathBuf::from(path).exists() == false {
        return Err("The path does not exist, please make sure the entered directory exists".to_string());
    };
    Ok(())
}

/// 是否为有效的目录路径
pub fn isValidDirectory(directory: String) -> Result<(), String> {
    let path = PathBuf::from(directory);
    if path.exists() == false {
        return Err("The directory does not exist, please make sure the entered directory exists".to_string());
    };
    if !path.is_dir() {
        return Err("The path is not a directory, please make sure that the entered path is a directory".to_string());
    };
    Ok(())
}

/// 是否为有效的路径（包括通配符）
pub fn isValidPathIncludeWildcard(path: String) -> Result<(), String> {
    let path = PathBuf::from(path);

    let fileName = path.file_name().unwrap().to_str().unwrap();
    if fileName.contains("*") || fileName.contains("?") {
        return if path.parent().unwrap().exists() {
            Ok(())
        } else {
            Err("The path does not exist, please make sure the entered path exists".to_string())
        };
    }
    if !path.exists() {
        return Err("The path does not exist, please make sure the entered path exists".to_string());
    }
    Ok(())
}

/// 是否为有效的驱动类别
pub fn isValidDriverClass(class: String) -> Result<(), String> {
    // HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Class\{f8ecafa6-66d1-41a5-899b-66585d7216b7}
    let driverClass = ["XboxComposite", "RDPDR", "DXGKrnl", "RemotePosDevice", "DigitalMediaDevices", "PrintQueue", "WCEUSBS", "SecurityAccelerator", "HidMsr", "SystemRecovery", "vhdmp", "fvevol", "fvevol", "USB", "ContentScreener", "Media Center Extender", "PnpPrinters", "Dot4", "Replication", "Dot4Print", "CDROM", "Computer", "DiskDrive", "Display", "FDC", "HDC", "Keyboard", "MEDIA", "Modem", "Monitor", "Mouse", "MTD", "MultiFunction", "Net", "NetClient", "NetService", "NetTrans", "PCMCIA", "Ports", "Printer", "SCSIAdapter", "System", "Unknown", "FloppyDisk", "HidLineDisplay", "Processor", "MultiPortSerial", "Memory", "SmartCardReader", "Sensor", "VolumeSnapshot", "SmrDisk", "ScmDisk", "SmrVolume", "ScmVolume", "Biometric", "Proximity", "AudioProcessingObject", "OposLegacyDevice", "SoftwareComponent", "FSFilterSystem", "XLGuard", "SoftwareDevice", "PerceptionSimulation", "PhysicalQuotaManagement", "1394", "Infrared", "Image", "TapeDrive", "BasicDisplay", "Volume", "ContinuousBackup", "Battery", "HIDClass", "HidCashDrawer", "61883", "RdpVideoMiniport", "QuotaManagement", "NetDriver", "TS_Generic", "USBDevice", "CopyProtection", "LegacyDriver", "SmartCard", "EhStorSilo", "XLWFP", "SDHost", "Encryption", "rdpbus", "AntiVirus", "RDCamera", "ActivityMonitor", "USBFunctionController", "AVC", "AudioEndpoint", "BarcodeScanner", "WSDPrintDevice", "POSPrinter", "Camera", "RDPDR", "CFSMetadataServer", "MediumChanger", "SecurityEnhancer", "Miracast", "SBP2", "HSM", "Holographic", "XnaComposite", "SecurityDevices", "SmartCardFilter", "Bluetooth", "Extension", "Infrastructure", "UCM", "WPD", "ComputeAccelerator", "Firmware", "Compression", "Virtualization", "OpenFileBackup", "Undelete"];
    for item in driverClass.iter() {
        if item.to_lowercase() == class.to_lowercase() { return Ok(()); }
    }
    return Err("The driver category is incorrect, please enter the correct driver category".to_string());
}
