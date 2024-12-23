pub mod quick;
pub mod script;

impl crate::meta::Patcher {
    pub fn run(
        &self,
        meta: &crate::meta::GameMeta,
        path: impl Into<std::path::PathBuf>,
    ) -> Result<(), crate::Error> {
        let path = path.into();
        match self {
            Self::Quick { method } => method.run(&path, meta),
            Self::Script { path } => {
                let script = std::fs::read_to_string(meta.patcher_dir.join(path))?;
                crate::patchers::script::run_patcher(script, meta, path)
            }
        }
    }
}
