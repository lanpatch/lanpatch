use rhai::{CustomType, TypeBuilder};

/// The metadata of the game.
#[derive(Debug, Clone, Copy, CustomType)]
pub struct GameMeta {
    pub steam: SteamMeta,
}

/// The Steam metadata of the game.
#[derive(Debug, Clone, Copy, CustomType)]
pub struct SteamMeta {
    /// The Steam AppID of the game.
    pub app_id: u32,
    /// The last confirmed working build depot.
    pub build_id: u32,
}
