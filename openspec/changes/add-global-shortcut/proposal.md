## Why

De popover open je nu alleen door op het icoon in de menubalk te klikken. Met een sneltoets kun je hem openen zonder je muis te pakken, vanuit elke app.

## What Changes

- de sneltoets ⌃⌥⌘M opent de popover, nog een keer drukken sluit hem
- werkt vanuit elke app, ook als de app geen focus heeft
- het rechtsklik-menu krijgt een vinkje 'Sneltoets ⌃ ⌥ ⌘ M', bij 'Toon in notch' en 'Start bij inloggen'
- vinkje uit: de sneltoets doet niets meer en andere apps kunnen hem weer gebruiken
- standaard aan, de keuze blijft bewaard na een herstart
- de sneltoets is vast, zelf een toets kiezen zit er niet in, want daar is een instellingenvenster voor nodig

Dit is change 5 van 6. Daarna volgt `add-track-notifications`.

## Capabilities

### New Capabilities
- `global-shortcut`: popover openen en sluiten met een vaste sneltoets, aan en uit te zetten vanuit het menu

### Modified Capabilities

## Impact

- `src-tauri/src/lib.rs`: sneltoets registreren, vinkje in het menu
- `src-tauri/src/settings.rs`: nieuw veld voor de sneltoets in `settings.json`
- nieuwe dependency `tauri-plugin-global-shortcut`
