use std::path::Path;

use lanpatcher::game::GameMeta;

fn main() {
    lanpatcher::scripts::run_patcher(
        "",
        GameMeta {
            steam: lanpatcher::game::SteamMeta {
                app_id: 12345,
                build_id: 67890,
            },
        },
        Path::new("C:\\Games\\MyGame"),
    )
    .unwrap();
    // Runtime::new().unwrap().block_on(async {
    //     let dir = std::path::Path::new("C:\\Games\\MyGame");
    //     let goldberg = lanpatch::goldberg::Goldberg::new(
    //         lanpatch::goldberg::Arch::X64,
    //         lanpatch::goldberg::Os::Windows,
    //     );
    //     let app_id = lanpatch::AppId(123456);
    //     lanpatch::goldberg::install(dir, goldberg, app_id)
    //         .await
    //         .unwrap();
    // });
}
