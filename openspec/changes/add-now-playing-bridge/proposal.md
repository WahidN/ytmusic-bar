## Why

De app weet nu niet wat er speelt. Alleen de pagina in de popover weet dat. Voor het notch-paneel (change 3) moet de app het nummer kennen en het afspelen kunnen bedienen zonder de popover te openen.

## What Changes

- de app leest titel, artiest, cover en speel-status uit YouTube Music
- de tooltip van het menubalk-icoon toont het huidige nummer
- het rechtsklik-menu krijgt 'Vorige', 'Afspelen'/'Pauzeren' en 'Volgende', boven 'Stop'
- de bediening werkt ook als de popover dicht is

## Capabilities

### New Capabilities
- `now-playing`: huidig nummer uitlezen en afspelen bedienen vanuit de app, buiten de pagina om

### Modified Capabilities

## Impact

- `src-tauri/src/lib.rs`: venster in code aanmaken met een init script, state voor het huidige nummer, uitgebreid menu
- nieuw script dat in de YouTube Music pagina draait
- nieuwe capability die music.youtube.com één eigen command laat aanroepen
- geen nieuwe dependencies
