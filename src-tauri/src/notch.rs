use objc2::MainThreadMarker;
use objc2_app_kit::{NSScreen, NSStatusWindowLevel, NSWindow, NSWindowCollectionBehavior};
use objc2_foundation::{NSPoint, NSRect, NSSize};
use serde::Serialize;
use tauri::WebviewWindow;

#[derive(Clone, Copy, Serialize)]
pub struct Notch {
    #[serde(skip)]
    center_x: f64,
    #[serde(skip)]
    screen_top: f64,
    pub width: f64,
    pub height: f64,
}

pub fn find(mtm: MainThreadMarker) -> Option<Notch> {
    NSScreen::screens(mtm).iter().find_map(|screen| {
        let height = screen.safeAreaInsets().top;
        if height <= 0.0 {
            return None;
        }
        let frame = screen.frame();
        let left = screen.auxiliaryTopLeftArea().size.width;
        let width = frame.size.width - left - screen.auxiliaryTopRightArea().size.width;
        Some(Notch {
            center_x: frame.origin.x + left + width / 2.0,
            screen_top: frame.origin.y + frame.size.height,
            width,
            height,
        })
    })
}

pub fn with_ns_window(window: &WebviewWindow, f: impl FnOnce(&NSWindow)) {
    if let Ok(ptr) = window.ns_window() {
        // Tauri geeft een geldige NSWindow pointer zolang het venster bestaat.
        f(unsafe { &*(ptr as *const NSWindow) });
    }
}

pub fn float_above_menu_bar(window: &NSWindow) {
    window.setLevel(NSStatusWindowLevel);
    window.setCollectionBehavior(
        NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::FullScreenAuxiliary
            | NSWindowCollectionBehavior::IgnoresCycle,
    );
}

// AppKit telt y van onder naar boven, dus de bovenrand van het scherm is het ankerpunt.
pub fn place(window: &NSWindow, notch: Notch, width: f64, height: f64) {
    let origin = NSPoint::new(notch.center_x - width / 2.0, notch.screen_top - height);
    window.setFrame_display(NSRect::new(origin, NSSize::new(width, height)), true);
}
