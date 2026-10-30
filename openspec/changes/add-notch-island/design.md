## Context

- Rust houdt het huidige nummer bij in `Mutex<NowPlaying>` en stuurt bij elke wijziging een `now-playing` event (change 2)
- `run_media_action` bedient YouTube Music via de handlers in de pagina
- de React-template in `src/` wordt nog nergens gebruikt

## Goals / Non-Goals

**Goals:**
- island dat voelt als de Dynamic Island: zwart, vloeiend, geen rand

**Non-Goals:**
- kiezen tussen icoon en notch (change 4)
- voortgangsbalk, volume, like-knop
- opnieuw positioneren als je een scherm aansluit of loskoppelt terwijl de app draait

## Decisions

- **Tweede Tauri-venster `island` met de React-app.** Transparant, geen rand, `skip_taskbar`, focusbaar zodat een klik ernaast het sluit. Laadt de lokale React-build, niet YouTube Music.
- **Notch-maat uit `NSScreen`.** `safeAreaInsets.top > 0` betekent notch. De breedte is de schermbreedte min `auxiliaryTopLeftArea` en `auxiliaryTopRightArea`. Via `objc2-app-kit`, dat al in de dependency tree zit via Tauri.
- **Window level boven de menubalk.** Tauri kent alleen `always_on_top`, dat is lager dan de menubalk. Via `ns_window()` zetten we `NSStatusWindowLevel` en de collection behavior `canJoinAllSpaces | stationary | fullScreenAuxiliary`.
- **Venster is zo groot als wat je ziet.** Een transparant venster vangt toch klikken op, dus een groot venster zou de menubalk blokkeren. Bij een klik maakt Rust het venster eerst groot en animeert React daarna de vorm. Bij een klik ernaast animeert React eerst terug en maakt Rust het venster daarna klein. Alternatief was `setIgnoresMouseEvents` aan en uit zetten, maar dan werkt klikken niet meer.
- **Data: event plus begin-state.** Bij laden haalt React de huidige state op met `get_now_playing`, daarna luistert het naar `now-playing`. Knoppen roepen `media_action` aan.
- **Capability alleen voor `island`.** `core:event:allow-listen`, `allow-get-now-playing`, `allow-media-action` en `allow-set-island-size`. De YouTube Music pagina krijgt er niets bij.
- **`macos-private-api` aan.** Zonder kan Tauri op macOS geen transparant venster maken.
- **Animatie met CSS transitions op breedte, hoogte en border-radius.** Easing dicht bij de spring van de Dynamic Island. Met `prefers-reduced-motion` direct wisselen.

## Risks / Trade-offs

- [Openen pakt de focus van de app waar je in zit] → net als de popover; openen is een bewuste klik. Hover is geprobeerd, maar WKWebView stuurt geen mouseleave in een venster zonder focus, dus het bleef open hangen.
- [Venster resize en CSS animatie lopen niet gelijk] → groot maken vóór de animatie en klein maken ná de animatie, dus de rand van het venster valt nooit binnen de zichtbare vorm
- [Private API] → geen App Store, wel gewoon te verspreiden als `.app`
- [Scherm wisselt tijdens gebruik] → positie wordt alleen bij start berekend, zie Non-Goals
