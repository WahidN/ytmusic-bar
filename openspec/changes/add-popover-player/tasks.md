## 1. Setup en login-test

- [x] 1.1 Rust installeren via rustup, check met `cargo --version`
- [x] 1.2 Tauri v2 project aanmaken met `pnpm create tauri-app` (React + TypeScript), check dat `pnpm tauri dev` een venster opent
- [x] 1.3 Venster music.youtube.com laten laden met de user agent van Safari, check dat inloggen met Google lukt zonder melding "browser niet veilig"

## 2. Menubalk en popover

- [x] 2.1 App uit het Dock halen (`ActivationPolicy::Accessory`) en een menubalk-icoon toevoegen, check dat het icoon in de menubalk staat en niet in het Dock
- [x] 2.2 Venster borderless maken en onder het icoon zetten met `tauri-plugin-positioner`, check dat de popover onder het icoon opent
- [x] 2.3 Klik op het icoon wisselt tussen tonen en verbergen, check met twee klikken
- [x] 2.4 Verbergen bij focusverlies, check dat een klik ernaast de popover sluit
- [x] 2.5 Rechtsklik-menu met 'Stop', check dat de app stopt

## 3. Controle

- [x] 3.1 Login na herstart, check: inloggen, app stoppen, opnieuw starten, nog ingelogd
- [x] 3.2 Doorspelen, check: nummer starten, popover dicht, nummer speelt tot het einde en het volgende start
- [x] 3.3 `cargo clippy`, `pnpm tsc --noEmit` en `pnpm tauri build` zonder fouten
