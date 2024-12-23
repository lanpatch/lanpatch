use tokio::runtime::Runtime;

mod rhai;

fn main() {
    rhai::do_thing().unwrap();
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
