use colored::*;
use lexopt::prelude::*;
use serde::Deserialize;
use std::process::{Command, Stdio};
use winreg::enums::*;
use winreg::RegKey;

const SCOOP_PACKAGES: &[&str] = &[
    "dotter", "fnm", "fzf", "gh", "gsudo", "sfk", "unison", "vcxsrv", "winscp", "zoxide",
];

const CHOCO_PACKAGES: &[&str] = &["miniconda3", "wezterm"];

const MANUAL_PACKAGES: &[(&str, &str)] = &[
    ("miniconda3", "conda"),
    ("svn", "svn"),
    ("git", "git"),
];

#[derive(Debug, Deserialize)]
struct ScoopApp {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ScoopBucket {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Source")]
    source: String,
}

#[derive(Debug, Deserialize)]
struct ScoopExport {
    apps: Vec<ScoopApp>,
    #[allow(dead_code)]
    buckets: Vec<ScoopBucket>,
}


#[derive(Debug)]
struct Args {
    install: bool,
    help: bool,
    #[allow(dead_code)]
    version: bool,
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut install = false;
    let mut help = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => help = true,
            Short('V') | Long("version") => {
                println!("check-prerequisites {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Long("install") => install = true,
            _ => return Err(arg.unexpected().into()),
        }
    }

    Ok(Args { install, help, version: false })
}

fn show_help() {
    println!("{}", "Zet'ohm Development Environment Prerequisites Checker".cyan().bold());
    println!();
    println!("{}", "USAGE:".yellow().bold());
    println!("    check-prerequisites [OPTIONS]");
    println!();
    println!("{}", "OPTIONS:".yellow().bold());
    println!("    -h, --help       Show this help");
    println!("    -V, --version    Show version information");
    println!("    --install        Install missing prerequisites automatically");
    println!();
    println!("{}", "EXAMPLES:".yellow().bold());
    println!("    check-prerequisites           # Check only");
    println!("    check-prerequisites --install # Check + install");
    println!();
    println!("{}", "PREREQUISITES CHECKED:".yellow().bold());
    println!("    • Windows Developer Mode");
    println!("    • Scoop (package manager)");
    println!("    • Chocolatey (package manager)");
    println!("    • Manual packages: {}", MANUAL_PACKAGES.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", "));
    println!("    • Scoop packages: {}", SCOOP_PACKAGES.join(", "));
    println!("    • Chocolatey packages: {}", CHOCO_PACKAGES.join(", "));
    println!();
    println!("{}", "NOTE: Some installations require administrator privileges.".red());
}

fn print_ok(msg: &str) {
    println!("✅ {}", msg.green());
}

fn print_warning(msg: &str) {
    println!("⚠️  {}", msg.yellow());
}

fn print_error(msg: &str) {
    println!("❌ {}", msg.red());
}

fn print_info(msg: &str) {
    println!("ℹ️  {}", msg.cyan());
}

fn print_section(msg: &str) {
    println!();
    println!("🔧 {}", msg.magenta().bold());
}


fn is_admin() -> bool {
    Command::new("net")
        .args(&["session"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn check_developer_mode() -> Result<bool, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppModelUnlock")?;
    let dev_mode: u32 = key.get_value("AllowDevelopmentWithoutDevLicense")?;
    Ok(dev_mode == 1)
}

fn enable_developer_mode() -> Result<(), Box<dyn std::error::Error>> {
    if !is_admin() {
        print_error("Administrator rights required to enable Developer Mode");
        return Err("Not admin".into());
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm.create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\AppModelUnlock")?;
    key.set_value("AllowDevelopmentWithoutDevLicense", &1u32)?;
    print_ok("Developer Mode enabled");
    Ok(())
}

fn check_command_exists(command: &str) -> bool {
    Command::new("where")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn get_manual_package_version(command: &str) -> Option<String> {
    let output = Command::new(command)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version_output = String::from_utf8_lossy(&output.stdout);
            // Extract first line and clean it up
            let version = version_output.lines().next()
                .unwrap_or("(installed)")
                .trim()
                .to_string();
            Some(version)
        }
        _ => None,
    }
}

fn install_scoop() -> Result<(), Box<dyn std::error::Error>> {
    print_info("Installing Scoop...");
    
    let output = Command::new("powershell")
        .args(&[
            "-ExecutionPolicy", "Bypass", "-Command",
            "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression"
        ])
        .output()?;
    
    if output.status.success() {
        print_ok("Scoop installed successfully");
        Ok(())
    } else {
        print_error("Error installing Scoop");
        Err("Scoop installation failed".into())
    }
}

fn install_chocolatey() -> Result<(), Box<dyn std::error::Error>> {
    if !is_admin() {
        print_error("Administrator rights required to install Chocolatey");
        return Err("Not admin".into());
    }

    print_info("Installing Chocolatey...");
    
    let output = Command::new("powershell")
        .args(&[
            "-ExecutionPolicy", "Bypass", "-Command",
            "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"
        ])
        .output()?;
    
    if output.status.success() {
        print_ok("Chocolatey installed successfully");
        Ok(())
    } else {
        print_error("Error installing Chocolatey");
        Err("Chocolatey installation failed".into())
    }
}

fn get_scoop_installed_packages() -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "scoop export"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("scoop export failed: {}", stderr).into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Debug output for troubleshooting
    if stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("scoop export returned empty output. stderr: {}", stderr).into());
    }
    
    let scoop_export: ScoopExport = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse scoop export JSON: {}. Output was: {}", e, stdout))?;
    
    let mut packages = std::collections::HashMap::new();
    for app in scoop_export.apps {
        // Skip apps that failed to install or have no version
        if let Some(version) = app.version {
            if !version.is_empty() {
                packages.insert(app.name, version);
            }
        }
    }
    
    Ok(packages)
}

fn get_scoop_package_version(package: &str, installed_packages: &std::collections::HashMap<String, String>) -> Option<String> {
    // First try to get version from scoop export
    if let Some(version) = installed_packages.get(package) {
        return Some(version.clone());
    }
    
    // Fallback: check if package is in PATH (but no version info)
    if check_command_exists(package) {
        return Some("(installed)".to_string());
    }
    
    None
}


fn install_scoop_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    print_info(&format!("Installing {}...", package));
    
    let output = Command::new("scoop")
        .args(&["install", package])
        .stdout(Stdio::null())
        .output()?;
    
    if output.status.success() {
        print_ok(&format!("{} installed", package));
        Ok(())
    } else {
        print_error(&format!("Error installing {}", package));
        Err(format!("{} installation failed", package).into())
    }
}

fn get_choco_installed_packages() -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    // Use chocolatey list to get package information
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "choco list"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;
    
    if !output.status.success() {
        return Err("choco list failed".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = std::collections::HashMap::new();
    
    // Parse lines like "packagename version"
    for line in stdout.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && 
           !trimmed.starts_with("Chocolatey") && 
           !trimmed.contains("packages installed") {
            
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let package_name = parts[0];
                let version = parts[1];
                result.insert(package_name.to_string(), version.to_string());
            }
        }
    }
    
    Ok(result)
}

fn get_choco_package_version(package: &str, installed_packages: &std::collections::HashMap<String, String>) -> Option<String> {
    installed_packages.get(package).cloned()
}


fn install_choco_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !is_admin() {
        print_error(&format!("Administrator rights required to install {}", package));
        return Err("Not admin".into());
    }

    print_info(&format!("Installing {}...", package));
    
    let output = Command::new("choco")
        .args(&["install", package, "-y"])
        .stdout(Stdio::null())
        .output()?;
    
    if output.status.success() {
        print_ok(&format!("{} installed", package));
        Ok(())
    } else {
        print_error(&format!("Error installing {}", package));
        Err(format!("{} installation failed", package).into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    if args.help {
        show_help();
        return Ok(());
    }

    print_section("Zet'ohm Prerequisites Check");
    print_info(&format!(
        "Mode: {}",
        if args.install { "Check + Install" } else { "Check only" }
    ));

    // Check admin rights
    let admin = is_admin();
    if admin {
        print_info("Administrator rights detected");
    } else {
        print_warning("No administrator rights (required for some installations)");
    }

    // Developer Mode
    print_section("Checking Developer Mode");
    match check_developer_mode() {
        Ok(true) => print_ok("Developer Mode enabled"),
        Ok(false) => {
            print_warning("Developer Mode disabled");
            if args.install {
                if let Err(_) = enable_developer_mode() {
                    print_error("Failed to enable Developer Mode");
                }
            }
        }
        Err(_) => print_warning("Could not check Developer Mode"),
    }

    // Scoop
    print_section("Checking Scoop");
    let scoop_installed = check_command_exists("scoop");
    if scoop_installed {
        print_ok("Scoop installed");
    } else {
        print_warning("Scoop not installed");
        if args.install {
            if let Err(_) = install_scoop() {
                print_error("Failed to install Scoop");
            }
        }
    }

    // Chocolatey
    print_section("Checking Chocolatey");
    let choco_installed = check_command_exists("choco");
    if choco_installed {
        print_ok("Chocolatey installed");
    } else {
        print_warning("Chocolatey not installed");
        if args.install {
            if let Err(_) = install_chocolatey() {
                print_error("Failed to install Chocolatey");
            }
        }
    }

    // Manual packages
    print_section("Checking Manual Packages");
    let mut manual_ok = 0;
    let mut manual_found = std::collections::HashSet::new();
    for (package, command) in MANUAL_PACKAGES {
        if let Some(version) = get_manual_package_version(command) {
            print_ok(&format!("{} {} installed", package, version));
            manual_ok += 1;
            manual_found.insert(package);
        } else {
            print_warning(&format!("{} not installed (manual installation required)", package));
        }
    }

    // Scoop packages
    print_section("Checking Scoop Packages");
    let mut scoop_ok = 0;
    if scoop_installed || check_command_exists("scoop") {
        let installed_packages = get_scoop_installed_packages().unwrap_or_else(|_| {
            print_warning("Could not get scoop export, using fallback detection");
            std::collections::HashMap::new()
        });
        
        for package in SCOOP_PACKAGES {
            if let Some(version) = get_scoop_package_version(package, &installed_packages) {
                print_ok(&format!("{} {} installed", package, version));
                scoop_ok += 1;
            } else {
                print_warning(&format!("{} not installed", package));
                if args.install {
                    if let Err(_) = install_scoop_package(package) {
                        print_error(&format!("Failed to install {}", package));
                    } else {
                        scoop_ok += 1;
                    }
                }
            }
        }
    } else {
        for package in SCOOP_PACKAGES {
            print_warning(&format!("{} not installed (Scoop required)", package));
        }
    }

    // Chocolatey packages
    print_section("Checking Chocolatey Packages");
    let mut choco_ok = 0;
    if choco_installed || check_command_exists("choco") {
        let installed_packages = get_choco_installed_packages().unwrap_or_else(|e| {
            print_warning(&format!("Could not get choco list: {}, using fallback detection", e));
            std::collections::HashMap::new()
        });
        
        for package in CHOCO_PACKAGES {
            // Skip if already found in manual packages
            if manual_found.contains(package) {
                print_info(&format!("{} already found via manual installation", package));
                choco_ok += 1;
                continue;
            }
            
            if let Some(version) = get_choco_package_version(package, &installed_packages) {
                print_ok(&format!("{} {} installed", package, version));
                choco_ok += 1;
            } else {
                print_warning(&format!("{} not installed", package));
                if args.install {
                    if let Err(_) = install_choco_package(package) {
                        print_error(&format!("Failed to install {}", package));
                    } else {
                        choco_ok += 1;
                    }
                }
            }
        }
    } else {
        for package in CHOCO_PACKAGES {
            print_warning(&format!("{} not installed (Chocolatey required)", package));
        }
    }

    // Summary
    print_section("Summary");
    print_info(&format!(
        "Developer Mode: {}",
        if check_developer_mode().unwrap_or(false) { "OK" } else { "NOK" }
    ));
    print_info(&format!(
        "Scoop: {}",
        if check_command_exists("scoop") { "OK" } else { "NOK" }
    ));
    print_info(&format!(
        "Chocolatey: {}",
        if check_command_exists("choco") { "OK" } else { "NOK" }
    ));
    print_info(&format!("Manual packages: {}/{}", manual_ok, MANUAL_PACKAGES.len()));
    print_info(&format!("Scoop packages: {}/{}", scoop_ok, SCOOP_PACKAGES.len()));
    print_info(&format!("Chocolatey packages: {}/{}", choco_ok, CHOCO_PACKAGES.len()));

    if !args.install {
        println!();
        print_info("To install missing prerequisites, use: check-prerequisites --install");
    }

    Ok(())
}