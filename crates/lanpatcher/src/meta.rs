use std::path::PathBuf;

use crate::patchers;
use rhai::{CustomType, TypeBuilder};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, CustomType,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct AppId(pub u32);

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Arch {
    X86,
    X64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Os {
    Windows,
    Linux,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Patcher {
    Quick {
        method: patchers::quick::Method,
    },
    Script {
        /// The path to the script file.
        ///
        /// Is relative to the metadata file.
        path: String,
    },
}

/// The metadata of the game.
#[derive(Debug, Clone, Deserialize, Serialize, CustomType)]
pub struct GameMeta {
    #[serde(skip)]
    pub patcher_dir: PathBuf,
    pub steam: SteamMeta,
    pub exe: ExecutableMeta,
    pub patcher: Patcher,
}

/// The Steam metadata of the game.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, CustomType)]
pub struct SteamMeta {
    /// The Steam AppID of the game.
    pub app_id: AppId,
    /// The last confirmed working build depot.
    pub build_id: u32,
}

/// The executable metadata of the game.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, CustomType)]
pub struct ExecutableMeta {
    /// The architecture of the executable.
    pub arch: Arch,
    /// The operating system of the executable.
    pub os: Os,
}
