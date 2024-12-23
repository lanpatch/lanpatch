use crate::{
    meta::{self, AppId},
    steam_api,
};
use std::path::Path;

static GOLDBERG_WIN_X86: &[u8] = include_bytes!("../../../goldberg_emu/experimental/steam_api.dll");
static GOLDBERG_WIN_X64: &[u8] =
    include_bytes!("../../../goldberg_emu/experimental/steam_api64.dll");

static GOLDBERG_LINUX_X86: &[u8] =
    include_bytes!("../../../goldberg_emu/linux/x86/libsteam_api.so");
static GOLDBERG_LINUX_X64: &[u8] =
    include_bytes!("../../../goldberg_emu/linux/x86_64/libsteam_api.so");

pub fn lib_data(version: steam_api::Version) -> &'static [u8] {
    match (version.arch, version.os) {
        (meta::Arch::X86, meta::Os::Windows) => GOLDBERG_WIN_X86,
        (meta::Arch::X64, meta::Os::Windows) => GOLDBERG_WIN_X64,
        (meta::Arch::X86, meta::Os::Linux) => GOLDBERG_LINUX_X86,
        (meta::Arch::X64, meta::Os::Linux) => GOLDBERG_LINUX_X64,
    }
}

/// Installs the Goldberg Emulator at the specified path.
///
/// # Arguments
///
/// * `path` - The path to install the `steam_api(64).dll`/`libsteam_api.so`.
/// * `version` - The version of the library to install.
/// * `app_id` - The AppID of the game.
pub fn install(
    path: &Path,
    version: steam_api::Version,
    app_id: AppId,
) -> Result<(), crate::Error> {
    tracing::info!(?path, "Installing Goldberg");

    let data = lib_data(version);

    std::fs::write(path, data)?;

    let dir = path.parent().unwrap();

    let app_id_path = dir.join("steam_appid.txt");
    std::fs::write(&app_id_path, app_id.0.to_string())?;

    // Goldberg disables non-LAN connections by default. This file disables that behavior.
    let disable_lan_only = dir.join("disable_lan_only.txt");
    std::fs::write(&disable_lan_only, "1")?;

    Ok(())
}
