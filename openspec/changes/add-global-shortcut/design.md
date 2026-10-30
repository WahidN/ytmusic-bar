## Context

- `toggle_popover` opent en sluit de popover al bij een klik op het icoon
- `settings.json` heeft nu alleen `showInNotch`, en `toggle_island` schrijft het bestand met alleen dat veld

## Goals / Non-Goals

**Non-Goals:**
- zelf een sneltoets kiezen
- sneltoetsen voor afspelen, pauzeren of volgende

## Decisions

- **`tauri-plugin-global-shortcut` v2.** De officiële plugin voor Tauri 2. Onder de motorkap gebruikt hij de Carbon hotkey API van macOS, die geen toegankelijkheid-toestemming vraagt. v3 is nog alpha en hoort bij Tauri 3.
- **⌃⌥⌘M, vast.** Vrijwel nergens in gebruik, dus geen botsing met bekende sneltoetsen. ⌥⌘M was makkelijker, maar dat is 'Alle vensters minimaliseren' in Finder en veel andere apps.
- **Alleen reageren op indrukken, niet op loslaten.** De plugin stuurt allebei. Anders gaat de popover open en meteen weer dicht.
- **Plek onder het icoon zelf uitrekenen met `tray.rect()`.** `tauri-plugin-positioner` kent het icoon pas na een hover of klik, dus een sneltoets direct na het opstarten opende de popover op de verkeerde plek. De rect wordt bij elke keer openen opgevraagd. De plugin is daarmee niet meer nodig.
- **Registreren en loslaten vanuit Rust.** Het vinkje leeft in het Rust-menu, dus de webview krijgt geen nieuwe capability.
- **Huidige settings in state.** Met een tweede veld kan `toggle_island` niet meer een nieuw `Settings` met één veld schrijven, want dan wist hij de sneltoets-keuze. De settings komen in een `Mutex<Settings>`. Elk vinkje past zijn eigen veld aan en schrijft daarna het hele bestand.
- **Een oud `settings.json` zonder het nieuwe veld blijft werken.** `#[serde(default)]` vult het ontbrekende veld met de standaard, dus aan.

## Risks / Trade-offs

- [Een andere app heeft ⌃⌥⌘M al] → registreren faalt, het vinkje gaat uit en de app start gewoon
- [De popover krijgt focus vanuit een andere app] → zo werkt de klik op het icoon ook al; de popover sluit weer als je ergens anders klikt
