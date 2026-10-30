## 1. Venster en script

- [x] 1.1 Venster in config op `create: false` zetten en in `setup` maken met `WebviewWindowBuilder::from_config`, check dat de popover werkt zoals in change 1
- [x] 1.2 Init script dat `setActionHandler` afvangt en metadata plus speel-status naar `now_playing` stuurt, check met een log in Rust bij elk nieuw nummer

## 2. Rust kant

- [x] 2.1 `now_playing` command met `Mutex<NowPlaying>` state, check dat de log titel, artiest, cover-URL en status toont
- [x] 2.2 Capability met `remote.urls` voor music.youtube.com en alleen `allow-now-playing`, check dat een ander command vanaf de pagina geweigerd wordt
- [x] 2.3 Tooltip van het icoon bijwerken bij elke wijziging, check met hover na een nieuw nummer

## 3. Menu

- [x] 3.1 'Vorige', 'Afspelen'/'Pauzeren' en 'Volgende' in het menu die de handlers aanroepen via `window.eval()`, check elke actie met de popover dicht
- [x] 3.2 Label 'Afspelen'/'Pauzeren' volgt de speel-status, check na pauzeren via de popover

## 4. Controle

- [x] 4.1 `cargo clippy`, `pnpm tsc --noEmit` en `pnpm tauri build` zonder fouten
