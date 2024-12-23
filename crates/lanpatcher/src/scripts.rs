use rhai::{CustomType, Engine, EvalAltResult, Scope, TypeBuilder};
use std::path::{Path, PathBuf};

pub fn run_patcher(
    rhai_src: impl AsRef<str>,
    meta: crate::game::GameMeta,
    path: impl Into<PathBuf>,
) -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.build_type::<crate::game::GameMeta>();
    engine.build_type::<crate::game::SteamMeta>();

    let ast = engine.compile(rhai_src)?;

    let mut scope = Scope::new();
    let result = engine.call_fn::<()>(&mut scope, &ast, "patch", (meta, path.into()))?;

    Ok(())
}
