use tauri::{AppHandle, Builder, Manager, Wry};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    Target, TargetKind,
};

pub fn register_plugins(app_builder: Builder<Wry>) -> Builder<Wry> {
    app_builder
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--silent"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app: &AppHandle<Wry>, argv: Vec<String>, cwd: String| -> () {
            log::trace!("Instance Detected. Executing with: {argv:?}, from: {cwd}");
            if argv.contains(&"roulette".to_owned()) {
                app.emit("open-seelenpad", ()).expect("Failed to emit open-settings event");
                return;
            }

            app.emit("open-settings", ()).expect("Failed to emit open-settings event");
        }))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .with_colors(ColoredLevelConfig {
                    error: Color::Red,
                    warn: Color::Yellow,
                    debug: Color::BrightGreen,
                    info: Color::BrightBlue,
                    trace: Color::White,
                })
                .level_for("tao", log::LevelFilter::Off)
                .build(),
        )
}
