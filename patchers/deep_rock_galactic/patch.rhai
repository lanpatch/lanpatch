fn patch(meta, path) {
    let dlls = find_steam_dlls(path);

    // should only be one DLL

    let dll = dlls[0];
    install_goldberg(dll,  meta.steam.app_id);
}
