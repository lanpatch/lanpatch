//! Common patchers to avoid copy-pasting the same scripts.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::meta::{Arch, GameMeta};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, clap::ValueEnum)]
pub enum Method {
    /// Replaces `steam_api(64).dll`/`libsteam_api.so` with the Goldberg emulator.
    Goldberg,
}

impl Method {
    pub fn run(self, path: &Path, meta: &GameMeta) -> Result<(), crate::Error> {
        tracing::info!(?self, "Running quick patcher");

        match self {
            Self::Goldberg => {
                let app_id = meta.steam.app_id;
                let dlls = crate::steam_api::find_steam_dlls(path, meta.exe.arch)?;
                for dll in dlls {
                    crate::goldberg::install(&dll.path, dll.version, app_id)?;
                }
                Ok(())
            }
        }
    }

    pub fn files_added(&self, path: &Path, arch: Arch) -> Vec<String> {
        match self {
            Self::Goldberg => {
                let steam_dlls = crate::steam_api::find_steam_dlls(path, arch).unwrap_or_default();

                let is_empty = steam_dlls.is_empty();

                let steam_dlls = steam_dlls.into_iter().map(|l| {
                    l.path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                });

                if is_empty {
                    vec![]
                } else {
                    let mut added = vec![
                        String::from("steam_appid.txt"),
                        String::from("disable_lan_only.txt"),
                        String::from("custom_broadcasts.txt"),
                    ];

                    added.extend(steam_dlls);

                    added
                }
            }
        }
    }
}
