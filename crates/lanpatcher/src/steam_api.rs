use std::path::{Path, PathBuf};

use rhai::{CustomType, TypeBuilder};
use serde::{Deserialize, Serialize};

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

    let mut walker_stack = vec![];
    let mut current_walker = std::fs::read_dir(game_root)?;

    loop {
        while let Some(entry) = current_walker.next() {
            let entry = entry?;

            let path = entry.path();

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
            } else {
                let old_walker = std::mem::replace(&mut current_walker, std::fs::read_dir(path)?);
                walker_stack.push(old_walker);
            }
        }
        if let Some(old_walker) = walker_stack.pop() {
            current_walker = old_walker;
        } else {
            break;
        }
    }

    Ok(found)
}
