use rhai::{CustomType, Dynamic, Engine, EvalAltResult, Scope, TypeBuilder};
use std::path::{Path, PathBuf};

use crate::{
    meta::{AppId, GameMeta},
    steam_api::Library,
};

pub fn run_patcher(
    rhai_src: impl AsRef<str>,
    path: impl Into<PathBuf>,
    meta: &crate::meta::GameMeta,
) -> Result<(), crate::Error> {
    let mut engine = Engine::new();

    engine.build_type::<crate::meta::GameMeta>();
    engine.build_type::<crate::meta::SteamMeta>();

    engine.build_type::<crate::steam_api::Library>();
    engine.build_type::<crate::steam_api::Version>();

    fn install_goldberg(
        dll_path: PathBuf,
        version: crate::steam_api::Version,
        app_id: AppId,
    ) -> Result<(), Box<EvalAltResult>> {
        if let Err(e) = crate::goldberg::install(&dll_path, version, app_id) {
            return Err(e.to_string().into());
        };
        Ok(())
    }
    engine.register_fn("install_goldberg", install_goldberg);

    fn find_steam_dlls(game_root: PathBuf, meta: GameMeta) -> Result<Dynamic, Box<EvalAltResult>> {
        match crate::steam_api::find_steam_dlls(&game_root, &meta) {
            Ok(paths) => Ok(paths.into()),
            Err(e) => Err(e.to_string().into()),
        }
    }
    engine.register_fn("find_steam_dlls", find_steam_dlls);

    let ast = engine.compile(rhai_src)?;

    let mut scope = Scope::new();
    let result = engine.call_fn::<()>(&mut scope, &ast, "patch", (path.into(), meta.clone()))?;

    Ok(())
}
