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
    ///
    /// If not provided, the patcher will attempt to determine the app ID from the game directory.
    #[arg(short, long)]
    app_id: Option<u32>,

    /// The path to the game directory.
    game_dir: PathBuf,
}

fn index_patchers(patchers: &mut HashMap<AppId, GameMeta>, root: &Path) -> Result<()> {
    tracing::info!(?root, "Indexing patchers");

    for entry in std::fs::read_dir(root)? {
        let entry = entry?;

        if !entry.file_type()?.is_dir() {
            tracing::warn!(?entry, "Skipping non-directory patcher entry");
        }

        let path = entry.path();

        let meta = std::fs::read_to_string(path.join("meta.toml"))?;

        let mut meta: GameMeta = toml::from_str(&meta)?;

        meta.patcher_dir = path;

        if let Some(old) = patchers.insert(meta.steam.app_id, meta) {
            tracing::warn!(?old, "Duplicate app ID found");
        };
    }

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    color_eyre::install()?;

    let out = real_main();

    if let Err(ref e) = out {
        tracing::error!(?e, "Error occurred");
    }

    tracing::info!("Press any key to close...");

    std::io::stdin().read_line(&mut String::new())?;

    out
}

fn real_main() -> Result<()> {
    let Args { app_id, game_dir } = Args::parse();

    let mut patchers = HashMap::new();

    if let Err(e) = index_patchers(
        &mut patchers,
        &std::env::current_exe()?
            .parent()
            .context("exe directory has no parent")?
            .join("patchers"),
    ) {
        tracing::warn!(?e, "Failed to index built-in patchers");
    }
    if let Err(e) = index_patchers(&mut patchers, Path::new("patchers")) {
        tracing::warn!(?e, "Failed to index patchers directory");
    }

    let game = if let Some(app_id) = app_id {
        patchers
            .get(&AppId(app_id))
            .context("couldn't find patcher for app")?
    } else {
        tracing::info!("App ID not provided, attempting to determine from game directory");

        patchers
            .values()
            .find(|game| game_dir.join(&game.exe.file).exists())
            .context("couldn't find patcher for game")?
    };

    tracing::info!(?game, "Got patcher");

    tracing::info!(?game_dir, "Patching...");

    game.patcher.run(&game_dir, game)?;

    tracing::info!("Done!");

    tracing::info!("Waiting for user input to exit...");

    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
