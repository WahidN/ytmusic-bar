## Context

- de popover is een webview met music.youtube.com, aangemaakt vanuit `tauri.conf.json`
- YouTube Music zet zelf `navigator.mediaSession`: metadata (titel, artiest, cover) en action handlers (play, pause, nexttrack, previoustrack)
- een remote pagina mag standaard geen Tauri commands aanroepen

## Goals / Non-Goals

**Goals:**
- één bron van waarheid in Rust voor het huidige nummer, klaar voor het notch-paneel

**Non-Goals:**
- voortgang, volume, like-knop
- media keys en Now Playing in het Control Center
- het notch-paneel zelf (change 3)

## Decisions

- **Lezen via `navigator.mediaSession`, niet via de DOM.** Dit is een standaard API die YouTube Music zelf vult voor het lockscreen en de headset. Class names in de DOM veranderen vaker. Speel-status komt uit de `play` en `pause` events van het media-element, met een capture listener op `document`. `mediaSession.playbackState` vult niet elke site.
- **Bedienen via de action handlers van de pagina.** Het init script vangt `MediaSession.prototype.setActionHandler` af en bewaart de handlers. Rust roept ze aan met `window.eval()`. Zo doet 'Volgende' precies wat de headset-knop doet. Alternatief was op knoppen in de DOM klikken, maar die selectors breken eerder.
- **Pagina stuurt, Rust luistert (push).** Het script stuurt bij elke wijziging een `now_playing` command. `window.eval()` kan geen waarde teruggeven, dus pollen vanuit Rust kan niet.
- **Venster in code aanmaken.** `tauri.conf.json` heeft geen veld voor een init script. Het venster blijft in de config staan met `create: false`, `WebviewWindowBuilder::from_config` voegt het script toe.
- **Capability met `remote.urls`.** Alleen `https://music.youtube.com/*`, alleen permission `allow-now-playing`. Geen `core:default` voor de remote pagina.
- **State in `Mutex<NowPlaying>` via `app.manage()`.** Het notch-paneel leest later dezelfde state.

## Risks / Trade-offs

- [YouTube Music stopt met `mediaSession`] → kans is klein, het lockscreen en AirPods hangen ervan af. Dan valt de app terug op 'YouTube Music' in de tooltip.
- [Pagina kan ons command aanroepen met nep-data] → het ergste gevolg is een verkeerde titel in de tooltip. Er is geen command dat iets op schijf of buiten de app doet.
- [Init script draait voor de pagina geladen is] → het script wacht op de eerste `metadata` en stuurt pas dan.
