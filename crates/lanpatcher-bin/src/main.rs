use color_eyre::eyre::Result;
use std::{collections::HashMap, path::Path};

use lanpatcher::meta::{AppId, GameMeta};

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

    let patchers = index_patchers(Path::new("patchers"))?;

    let drg = patchers.get(&AppId(548430)).unwrap();

    tracing::info!(?drg, "Got patcher");

    let mut args = std::env::args().skip(1);
    let game_dir = args.next().unwrap();

    tracing::info!(game_dir, "Patching...");

    drg.patcher.run(drg, Path::new(&game_dir))?;

    Ok(())
}
