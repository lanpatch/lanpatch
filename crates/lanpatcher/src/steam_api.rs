use std::path::{Path, PathBuf};

use rhai::{CustomType, TypeBuilder};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{meta, Error};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, CustomType)]
pub struct Version {
    pub arch: meta::Arch,
    pub os: meta::Os,
}

impl Version {
    pub fn new(arch: meta::Arch, os: meta::Os) -> Self {
        Self { arch, os }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, CustomType)]
pub struct Library {
    pub path: PathBuf,
    pub version: Version,
}

/// Walks through the game directory and finds the `steam_api(64).dll`/`libsteam_api.so` files.
pub fn find_steam_dlls(game_root: &Path, meta: &meta::GameMeta) -> Result<Vec<Library>, Error> {
    let mut found = Vec::with_capacity(1);

    tracing::info!(?game_root, "Searching for Steam DLLs");

    for entry in WalkDir::new(game_root) {
        let entry = entry?;

        let path = entry.path().to_path_buf();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                match file_name {
                    x if x == "steam_api.dll" => {
                        found.push(Library {
                            path,
                            version: Version::new(meta::Arch::X86, meta::Os::Windows),
                        });
                    }
                    x if x == "steam_api64.dll" => {
                        found.push(Library {
                            path,
                            version: Version::new(meta::Arch::X64, meta::Os::Windows),
                        });
                    }
                    x if x == "libsteam_api.so" => {
                        found.push(Library {
                            path,
                            version: Version::new(meta.exe.arch, meta::Os::Linux),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    tracing::info!(?found, "Found Steam DLLs");

    Ok(found)
}
