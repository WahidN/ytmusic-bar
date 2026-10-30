## 1. Settings

- [x] 1.1 Settings in een `Mutex<Settings>` in state, `toggle_island` past alleen zijn eigen veld aan, check dat 'Toon in notch' uitzetten `globalShortcut` in `settings.json` laat staan
- [x] 1.2 Veld `globalShortcut`, standaard aan, check met een oud `settings.json` zonder het veld dat de app start met het vinkje aan

## 2. Sneltoets

- [x] 2.1 `tauri-plugin-global-shortcut` toevoegen en ⌃⌥⌘M registreren als de keuze aan staat, alleen bij indrukken `toggle_popover`, check open en dicht vanuit een andere app en dat ingedrukt houden niet knippert
- [x] 2.2 `CheckMenuItem` 'Sneltoets ⌃ ⌥ ⌘ M' onder 'Start bij inloggen', klik registreert of geeft vrij en slaat op, check dat ⌃⌥⌘M niets doet met het vinkje uit
- [x] 2.3 Registreren mislukt: vinkje uit, geen crash, check door ⌃⌥⌘M tijdelijk twee keer te registreren
- [x] 2.4 Herstart met het vinkje uit, check dat de sneltoets uit blijft

## 3. Controle

- [x] 3.1 `cargo clippy`, `pnpm tsc --noEmit` en `pnpm tauri build` zonder fouten
