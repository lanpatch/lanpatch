use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod game;
pub mod goldberg;
pub mod scripts;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Walks through the game directory and finds the `steam_api(64).dll`/`libsteam_api.so` files.
pub async fn find_steam_dlls(game_root: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut found = Vec::with_capacity(1);

    let mut walker_stack = vec![];
    let mut current_walker = tokio::fs::read_dir(game_root).await?;

    loop {
        while let Some(entry) = current_walker.next_entry().await? {
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name == "steam_api.dll"
                        || file_name == "steam_api64.dll"
                        || file_name == "libsteam_api.so"
                    {
                        found.push(path);
                    }
                }
            } else {
                let old_walker =
                    std::mem::replace(&mut current_walker, tokio::fs::read_dir(path).await?);
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
