## Why

Het island over de notch staat nu altijd aan, en na een herstart van de Mac moet je de app zelf weer openen. Beide wil je kunnen kiezen zonder een instellingenvenster.

## What Changes

- het rechtsklik-menu van het menubalk-icoon krijgt twee vinkjes, tussen de bediening en 'Stop': 'Toon in notch' en 'Start bij inloggen'
- 'Toon in notch' uit: het island verdwijnt meteen, aan: het komt meteen terug
- 'Toon in notch' staat standaard aan en blijft bewaard na een herstart
- zonder notch is 'Toon in notch' grijs, want er is niets aan of uit te zetten
- 'Start bij inloggen' aan: de app start vanzelf als je inlogt op de Mac, standaard uit
- het menubalk-icoon blijft altijd, anders kun je de popover, het menu en 'Stop' niet meer bereiken

Dit is change 4. Daarna volgen, elk als eigen change:

5. `add-global-shortcut`: sneltoets om de popover te openen, met een vinkje om hem uit te zetten
6. `add-track-notifications`: melding bij een nieuw nummer, met een vinkje om hem uit te zetten

## Capabilities

### New Capabilities
- `settings`: island en start bij inloggen aan- en uitzetten vanuit het menu, bewaard tussen herstarts

### Modified Capabilities

## Impact

- `src-tauri/src/lib.rs`: vinkjes in het menu, island tonen of verbergen, keuze lezen bij start
- nieuw klein bestand `settings.json` in de app-configmap
- nieuwe dependency `tauri-plugin-autostart` voor 'Start bij inloggen'
