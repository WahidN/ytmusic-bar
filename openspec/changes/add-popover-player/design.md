## Context

- nieuw project, nog geen code
- YouTube Music heeft geen officiële API om af te spelen, dus de site draait in een webview
- Tauri op macOS gebruikt WKWebView, dezelfde engine als Safari

## Goals / Non-Goals

**Goals:**
- het geraamte waar change 2 tot en met 4 op bouwen

**Non-Goals:**
- eigen bediening of nummer-info buiten de site (change 2)
- notch (change 3) en instellingen (change 4)
- media keys, Now Playing in het Control Center, auto-start bij inloggen

## Decisions

- **Popover is een borderless Tauri-venster, geen `NSPopover`.** Tauri heeft geen popover. Een venster zonder rand, `always_on_top` en `skip_taskbar` doet hetzelfde. Positie via `tauri-plugin-positioner` (`TrayBottomCenter`).
- **Venster verbergen, niet sluiten.** Bij `Focused(false)` en een tweede klik gaat het venster op `hide()`. De webview blijft leven, daarom speelt de muziek door en laadt de pagina niet opnieuw.
- **De webview laadt direct `https://music.youtube.com`.** Geen eigen React-pagina eromheen. React blijft in het project voor de notch-UI in change 3.
- **User agent van Safari.** Google blokkeert inloggen in sommige embedded browsers. Met de user agent van Safari ziet Google een gewone browser. Alternatief was inloggen in een apart systeem-browservenster, maar dan komen de cookies niet in de webview.
- **Login via de standaard data store van WKWebView.** Die is per app-bundle en bewaart cookies op schijf. Geen eigen opslag nodig.
- **`ActivationPolicy::Accessory`.** Haalt de app uit het Dock en de app-switcher.

## Risks / Trade-offs

- [Google blokkeert de login toch] → task 1.3 test dit als eerste, voor de rest gebouwd wordt. Werkt het niet, dan eerst opties voorleggen voor er verder gebouwd wordt.
- [macOS pauzeert een verborgen webview] → audio loopt door in een verborgen WKWebView, timers kunnen trager gaan. Task 3.2 test een heel nummer met de popover dicht.
- [YouTube verandert de site] → in deze change raakt dat alleen de site zelf, de app leest nog niets uit de pagina
