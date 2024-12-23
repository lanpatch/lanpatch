use rhai::{CustomType, Engine, EvalAltResult, Scope, TypeBuilder};
use std::path::{Path, PathBuf};

use crate::{
    meta::{AppId, GameMeta},
    steam_api::Library,
};

pub fn run_patcher(
    rhai_src: impl AsRef<str>,
    meta: &crate::meta::GameMeta,
    path: impl Into<PathBuf>,
) -> Result<(), crate::Error> {
    let mut engine = Engine::new();

    engine.build_type::<crate::meta::GameMeta>();
    engine.build_type::<crate::meta::SteamMeta>();

    engine.build_type::<crate::steam_api::Library>();
    engine.build_type::<crate::steam_api::Version>();

    fn install_goldberg(
        path: &Path,
        version: crate::steam_api::Version,
        app_id: u32,
    ) -> Result<(), Box<EvalAltResult>> {
        if let Err(e) = crate::goldberg::install(path, version, AppId(app_id)) {
            return Err(e.to_string().into());
        };
        Ok(())
    }
    engine.register_fn("install_goldberg", install_goldberg);

    fn find_steam_dlls(
        game_root: &Path,
        meta: GameMeta,
    ) -> Result<Vec<Library>, Box<EvalAltResult>> {
        match crate::steam_api::find_steam_dlls(game_root, &meta) {
            Ok(paths) => Ok(paths),
            Err(e) => Err(e.to_string().into()),
        }
    }
    engine.register_fn("find_steam_dlls", find_steam_dlls);

    let ast = engine.compile(rhai_src)?;

    let mut scope = Scope::new();
    let result = engine.call_fn::<()>(&mut scope, &ast, "patch", (meta.clone(), path.into()))?;

    Ok(())
}
