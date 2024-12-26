use clap::Parser;
use color_eyre::eyre::{ContextCompat, Result};
use sha2::Digest;
use std::{
    collections::{BTreeSet, HashMap},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use lanpatch::{
    meta::{AppId, Arch, ExecutableMeta, GameMeta, Os, Patcher, SteamMeta},
    patchers::quick,
};

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

    /// The method to use if the app ID couldn't be determined.
    ///
    /// The Goldberg app ID will be determined by a
    /// hash of the file names.
    #[arg(short, long, default_value = "goldberg")]
    method: quick::Method,
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
    let Args {
        app_id,
        game_dir,
        method,
    } = Args::parse();

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
            .clone()
    } else {
        tracing::info!("App ID not provided, attempting to determine from game directory");

        match patchers
            .values()
            .find(|game| game_dir.join(&game.exe.file).exists())
        {
            Some(game) => game.clone(),
            None => {
                tracing::warn!("Couldn't determine app ID from game directory");
                tracing::info!(default_method = ?method, "Using default method");

                let mut file_names = WalkDir::new(&game_dir)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.file_name().to_string_lossy().to_string())
                    .collect::<BTreeSet<_>>();

                file_names.extend(method.files_added(&game_dir, Arch::X64));

                let mut hasher = sha2::Sha256::new();
                for file_name in file_names {
                    hasher.update(file_name.as_bytes());
                }
                let hash = hasher.finalize();

                tracing::info!(?hash, "Hashed file names");

                let app_id: u32 = u32::from_be_bytes(hash[..4].try_into().unwrap());

                tracing::info!(?app_id, "Generated app ID");

                let app_id = AppId(app_id);

                GameMeta {
                    steam: SteamMeta {
                        app_id,
                        build_id: 0,
                    },
                    exe: ExecutableMeta {
                        file: String::new(),
                        arch: Arch::X64,
                        os: Os::Windows,
                    },
                    patcher: Patcher::Quick { method },
                    patcher_dir: Default::default(),
                }
            }
        }
    };

    tracing::info!(?game, "Got patcher");

    tracing::info!(?game_dir, "Patching...");

    game.patcher.run(&game_dir, &game)?;

    tracing::info!("Done!");

    Ok(())
}
