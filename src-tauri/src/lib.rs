mod notch;
mod settings;

use std::sync::Mutex;

use notch::Notch;
use objc2::MainThreadMarker;
use serde::{Deserialize, Serialize};
use tauri::{
    image::Image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    ActivationPolicy, AppHandle, Emitter, Manager, State, WebviewWindowBuilder, WindowEvent, Wry,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_positioner::{Position, WindowExt};

const TRAY_ID: &str = "main";
const MEDIA_ACTIONS: [&str; 4] = ["play", "pause", "nexttrack", "previoustrack"];

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NowPlaying {
    title: String,
    artist: String,
    artwork: String,
    is_playing: bool,
}

struct PlayPauseItem(MenuItem<Wry>);

struct SettingsItems {
    show_in_notch: CheckMenuItem<Wry>,
    launch_at_login: CheckMenuItem<Wry>,
    global_shortcut: CheckMenuItem<Wry>,
}

fn popover_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT | Modifiers::SUPER), Code::KeyM)
}

#[tauri::command]
fn now_playing(app: AppHandle, track: NowPlaying) {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_tooltip(Some(format!("{} · {}", track.title, track.artist)));
    }
    let label = if track.is_playing { "Pauzeren" } else { "Afspelen" };
    let _ = app.state::<PlayPauseItem>().0.set_text(label);
    let _ = app.emit("now-playing", &track);
    *app.state::<Mutex<NowPlaying>>().lock().unwrap() = track;
}

#[tauri::command]
fn get_now_playing(state: State<Mutex<NowPlaying>>) -> NowPlaying {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn get_notch(notch: State<Notch>) -> Notch {
    *notch
}

#[tauri::command]
fn media_action(app: AppHandle, action: String) {
    if MEDIA_ACTIONS.contains(&action.as_str()) {
        run_media_action(&app, &action);
    }
}

#[tauri::command]
fn set_island_size(app: AppHandle, notch: State<Notch>, width: f64, height: f64) {
    let notch = *notch;
    if let Some(island) = app.get_webview_window("island") {
        let _ = app.run_on_main_thread(move || {
            notch::with_ns_window(&island, |window| notch::place(window, notch, width, height));
        });
    }
}

fn create_island(app: &AppHandle, notch: Notch, show: bool) -> tauri::Result<()> {
    let island = WebviewWindowBuilder::from_config(app, &app.config().app.windows[1])?.build()?;
    notch::with_ns_window(&island, |window| {
        notch::float_above_menu_bar(window);
        notch::place(window, notch, notch.width, notch.height);
    });
    if show {
        island.show()?;
    }
    Ok(())
}

// Het vinkje is bij een klik al omgezet, dus de nieuwe waarde lezen we eraf.
fn toggle_island(app: &AppHandle) {
    let show = app.state::<SettingsItems>().show_in_notch.is_checked().unwrap_or(true);
    if let Some(island) = app.get_webview_window("island") {
        let _ = if show { island.show() } else { island.hide() };
    }
    settings::update(app, |settings| settings.show_in_notch = show);
}

fn toggle_global_shortcut(app: &AppHandle) {
    let item = &app.state::<SettingsItems>().global_shortcut;
    let shortcuts = app.global_shortcut();
    let enabled = if item.is_checked().unwrap_or(false) {
        shortcuts.register(popover_shortcut()).is_ok()
    } else {
        let _ = shortcuts.unregister(popover_shortcut());
        false
    };
    let _ = item.set_checked(enabled);
    settings::update(app, |settings| settings.global_shortcut = enabled);
}

fn toggle_launch_at_login(app: &AppHandle) {
    let item = &app.state::<SettingsItems>().launch_at_login;
    let autolaunch = app.autolaunch();
    let _ = if item.is_checked().unwrap_or(false) {
        autolaunch.enable()
    } else {
        autolaunch.disable()
    };
    let _ = item.set_checked(autolaunch.is_enabled().unwrap_or(false));
}

fn run_media_action(app: &AppHandle, action: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval(format!("window.__ytmBar?.run('{action}')"));
    }
}

fn toggle_popover(app: &AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    if window.is_visible().unwrap_or(false) {
        let _ = window.hide();
    } else {
        let _ = window.move_window(Position::TrayBottomCenter);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_popover(app);
                    }
                })
                .build(),
        )
        .manage(Mutex::new(NowPlaying::default()))
        .invoke_handler(tauri::generate_handler![
            now_playing,
            get_now_playing,
            get_notch,
            media_action,
            set_island_size
        ])
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);

            WebviewWindowBuilder::from_config(app.handle(), &app.config().app.windows[0])?
                .initialization_script(include_str!("bridge.js"))
                .build()?;

            let settings = settings::load(app.handle());
            let mtm = MainThreadMarker::new().ok_or("setup must run on the main thread")?;
            let notch = notch::find(mtm);
            if let Some(notch) = notch {
                app.manage(notch);
                create_island(app.handle(), notch, settings.show_in_notch)?;
            }

            let previous = MenuItem::with_id(app, "previous", "Vorige", true, None::<&str>)?;
            let play_pause = MenuItem::with_id(app, "play_pause", "Afspelen", true, None::<&str>)?;
            let next = MenuItem::with_id(app, "next", "Volgende", true, None::<&str>)?;
            let show_in_notch = CheckMenuItem::with_id(
                app,
                "show_in_notch",
                "Toon in notch",
                notch.is_some(),
                settings.show_in_notch,
                None::<&str>,
            )?;
            let launch_at_login = CheckMenuItem::with_id(
                app,
                "launch_at_login",
                "Start bij inloggen",
                true,
                app.autolaunch().is_enabled().unwrap_or(false),
                None::<&str>,
            )?;
            let global_shortcut = CheckMenuItem::with_id(
                app,
                "global_shortcut",
                "Sneltoets ⌃ ⌥ ⌘ M",
                true,
                settings.global_shortcut
                    && app.global_shortcut().register(popover_shortcut()).is_ok(),
                None::<&str>,
            )?;
            let separator = PredefinedMenuItem::separator(app)?;
            let settings_separator = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Stop", true, None::<&str>)?;
            let menu = Menu::with_items(
                app,
                &[
                    &previous,
                    &play_pause,
                    &next,
                    &separator,
                    &show_in_notch,
                    &launch_at_login,
                    &global_shortcut,
                    &settings_separator,
                    &quit,
                ],
            )?;
            app.manage(PlayPauseItem(play_pause));
            app.manage(SettingsItems { show_in_notch, launch_at_login, global_shortcut });
            app.manage(Mutex::new(settings));

            TrayIconBuilder::with_id(TRAY_ID)
                .icon(Image::from_bytes(include_bytes!("../icons/tray.png"))?)
                .icon_as_template(true)
                .tooltip("YouTube Music")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "previous" => run_media_action(app, "previoustrack"),
                    "next" => run_media_action(app, "nexttrack"),
                    "play_pause" => {
                        let is_playing = app.state::<Mutex<NowPlaying>>().lock().unwrap().is_playing;
                        run_media_action(app, if is_playing { "pause" } else { "play" });
                    }
                    "show_in_notch" => toggle_island(app),
                    "launch_at_login" => toggle_launch_at_login(app),
                    "global_shortcut" => toggle_global_shortcut(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_popover(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Verbergen in plaats van sluiten, zodat de webview blijft leven en de muziek doorspeelt.
            if let (WindowEvent::Focused(false), "main") = (event, window.label()) {
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
