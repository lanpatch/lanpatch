use std::path::Path;

pub mod quick;
pub mod script;

impl crate::meta::Patcher {
    pub fn run(
        &self,
        game_path: impl Into<std::path::PathBuf>,
        meta: &crate::meta::GameMeta,
    ) -> Result<(), crate::Error> {
        let game_path = game_path.into();
        match self {
            Self::Quick { method } => method.run(&game_path, meta),
            Self::Script { path: script_path } => {
                let script_path = Path::new(&script_path);

                let script = std::fs::read_to_string(if script_path.is_absolute() {
                    script_path.to_path_buf()
                } else {
                    meta.patcher_dir.join(script_path)
                })?;

                crate::patchers::script::run_patcher(script, game_path, meta)
            }
        }
    }
}
