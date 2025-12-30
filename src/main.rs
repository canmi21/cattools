/* src/main.rs */

mod api;
mod cli;
mod config;
mod constants;
mod error;
mod menu;
mod modules;
mod uci;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;
use crate::menu::{Menu, MenuOption, UtilitiesMenuOption};
use crate::utils::system::{check_openwrt, check_root, patch_banner_domains, patch_catwrt_release};

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Check root privileges
    check_root().context("Root permission check failed")?;

    // Check OpenWrt system
    check_openwrt().context("OpenWrt system check failed")?;

    // Parse CLI arguments
    let _args = Cli::parse();

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    // Apply system patches
    let _ = patch_catwrt_release();
    let _ = patch_banner_domains();

    // Main loop
    loop {
        match Menu::show().context("Failed to show menu")? {
            MenuOption::SetIp => {
                if let Err(e) = modules::network::set_ip_address(&config) {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::NetworkWizard => {
                if let Err(e) = modules::network::network_wizard() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::ApplyRepo => {
                if let Err(e) = modules::package::apply_repo() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::NetworkDiagnostic => {
                if let Err(e) = modules::diagnostic::network_diagnostic() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::SystemDebug => {
                if let Err(e) = modules::diagnostic::system_debug() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::CheckUpdate => {
                if let Err(e) = modules::update::check_update() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::SystemUpgrade => {
                if let Err(e) = modules::update::system_upgrade() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::PackageBackup => {
                if let Err(e) = modules::package::package_backup_restore_menu() {
                    eprintln!("错误: {}", e);
                }
            }
            MenuOption::UtilitiesMenu => {
                handle_utilities_menu()?;
            }
            MenuOption::Exit => {
                println!("Done!");
                break;
            }
        }
    }

    Ok(())
}

fn handle_utilities_menu() -> Result<()> {
    loop {
        match Menu::show_utilities().context("Failed to show utilities menu")? {
            UtilitiesMenuOption::ConfigureMihomo => {
                if let Err(e) = modules::advanced::configure_mihomo() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::ConfigureTailscale => {
                if let Err(e) = modules::advanced::configure_tailscale() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::ConfigureLeigod => {
                if let Err(e) = modules::advanced::configure_leigod() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::ConfigureTtyd => {
                if let Err(e) = modules::advanced::configure_ttyd() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::InstallIpk => {
                if let Err(e) = modules::package::install_ipk() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::DeploySslCert => {
                if let Err(e) = modules::advanced::deploy_ssl_cert() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::ResetPassword => {
                if let Err(e) = modules::advanced::reset_root_password() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::SystemReset => {
                if let Err(e) = modules::advanced::system_reset() {
                    eprintln!("错误: {}", e);
                }
            }
            UtilitiesMenuOption::Back => {
                break;
            }
        }
    }
    Ok(())
}
