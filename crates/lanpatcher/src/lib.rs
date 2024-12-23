use rhai::EvalAltResult;
use thiserror::Error;

pub mod goldberg;
pub mod meta;
pub mod patchers;
pub mod steam_api;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("rhai eval error: {0}")]
    Rhai(#[from] Box<EvalAltResult>),
    #[error("error parsing script: {0}")]
    RhaiParse(#[from] rhai::ParseError),
}
