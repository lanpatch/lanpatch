//! Common patchers to avoid copy-pasting the same scripts.

use serde::{Deserialize, Serialize};

use crate::meta::GameMeta;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Method {
    /// Replaces `steam_api(64).dll`/`libsteam_api.so` with the Goldberg emulator.
    Goldberg,
}

impl Method {
    pub fn run(self, path: &std::path::Path, meta: &GameMeta) -> Result<(), crate::Error> {
        match self {
            Self::Goldberg => {
                let app_id = meta.steam.app_id;
                let dlls = crate::steam_api::find_steam_dlls(path, meta)?;
                for dll in dlls {
                    crate::goldberg::install(&dll.path, dll.version, app_id)?;
                }
                Ok(())
            }
        }
    }
}
