use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000100000000)
            .set_version_info(winres::VersionInfo::FILEVERSION, 0x0001000100000000)
            .set("ProductName", "Zet'ohm Prerequisites Checker")
            .set("FileDescription", "Zet'ohm Development Environment Prerequisites Checker")
            .set("CompanyName", "Zet'ohm")
            .set("LegalCopyright", "© 2025 Zet'ohm")
            .set("ProductVersion", "1.0.1.0")
            .set("FileVersion", "1.0.1.0")
            .set("OriginalFilename", "check-prerequisites.exe")
            .set("InternalName", "check-prerequisites")
            .compile()
            .unwrap();
    }
}