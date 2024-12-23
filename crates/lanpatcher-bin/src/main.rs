use clap::Parser;
use color_eyre::eyre::{ContextCompat, Result};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use lanpatcher::meta::{AppId, GameMeta};

/// A patcher for Steam games that allows for LAN-only play.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The app ID of the game to patch.
    #[arg(short, long)]
    app_id: u32,

    /// The path to the game directory.
    #[arg(short, long)]
    game_dir: PathBuf,
}

fn index_patchers(root: &Path) -> Result<HashMap<AppId, GameMeta>> {
    tracing::info!(?root, "Indexing patchers");

    let mut patchers = HashMap::new();

    for entry in std::fs::read_dir(root)? {
        let entry = entry?;

        if !entry.file_type()?.is_dir() {
            tracing::warn!(?entry, "Skipping non-directory patcher entry");
        }

        let path = entry.path();

        let meta = std::fs::read_to_string(path.join("meta.toml"))?;

        let mut meta: GameMeta = toml::from_str(&meta)?;

        meta.patcher_dir = path;

        patchers.insert(meta.steam.app_id, meta);
    }

    Ok(patchers)
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    color_eyre::install()?;

    let Args { app_id, game_dir } = Args::parse();

    let patchers = index_patchers(Path::new("patchers"))?;

    let drg = patchers
        .get(&AppId(app_id))
        .context("couldn't find patcher for app")?;

    tracing::info!(?drg, "Got patcher");

    tracing::info!(?game_dir, "Patching...");

    drg.patcher.run(&game_dir, drg)?;

    Ok(())
}
