fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&[
                "now_playing",
                "get_now_playing",
                "get_notch",
                "media_action",
                "set_island_size",
            ])),
    )
    .expect("failed to run tauri-build");
}
