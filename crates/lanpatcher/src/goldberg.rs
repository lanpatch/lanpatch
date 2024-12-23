use std::path::Path;

static GOLDBERG_WIN_X86: &[u8] = include_bytes!("../../../goldberg_emu/experimental/steam_api.dll");
static GOLDBERG_WIN_X64: &[u8] =
    include_bytes!("../../../goldberg_emu/experimental/steam_api64.dll");

static GOLDBERG_LINUX_X86: &[u8] =
    include_bytes!("../../../goldberg_emu/linux/x86/libsteam_api.so");
static GOLDBERG_LINUX_X64: &[u8] =
    include_bytes!("../../../goldberg_emu/linux/x86_64/libsteam_api.so");

#[derive(Debug, Clone, Copy)]
pub enum Arch {
    X86,
    X64,
}

#[derive(Debug, Clone, Copy)]
pub enum Os {
    Windows,
    Linux,
}

#[derive(Debug, Clone, Copy)]
pub struct Goldberg {
    pub arch: Arch,
    pub os: Os,
}

impl Goldberg {
    pub fn new(arch: Arch, os: Os) -> Self {
        Self { arch, os }
    }

    pub fn get_lib_data(self) -> &'static [u8] {
        match (self.os, self.arch) {
            (Os::Windows, Arch::X86) => GOLDBERG_WIN_X86,
            (Os::Windows, Arch::X64) => GOLDBERG_WIN_X64,
            (Os::Linux, Arch::X86) => GOLDBERG_LINUX_X86,
            (Os::Linux, Arch::X64) => GOLDBERG_LINUX_X64,
        }
    }
}

/// Installs the Goldberg Emulator at the specified path.
///
/// # Arguments
///
/// * `path` - The path to install the `steam_api(64).dll`/`libsteam_api.so`, and `steam_appid.txt`.
pub async fn install(dir: &Path, goldberg: Goldberg, app_id: u32) -> Result<(), crate::Error> {
    tracing::info!(?dir, "Installing Goldberg");

    let data = goldberg.get_lib_data();

    let file_name = match goldberg.os {
        Os::Windows => match goldberg.arch {
            Arch::X86 => "steam_api.dll",
            Arch::X64 => "steam_api64.dll",
        },
        Os::Linux => "libsteam_api.so",
    };

    let path = dir.join(file_name);

    tokio::fs::write(&path, data).await?;

    let app_id_path = dir.join("steam_appid.txt");
    tokio::fs::write(&app_id_path, app_id.to_string()).await?;

    // Goldberg disables non-LAN connections by default. This file disables that behavior.
    let disable_lan_only = dir.join("disable_lan_only.txt");
    tokio::fs::write(&disable_lan_only, "1").await?;

    Ok(())
}
