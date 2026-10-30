## Context

- het island venster wordt bij start aangemaakt als er een notch is, en dan meteen getoond
- het menu wordt in `setup` gebouwd, met de bediening en 'Stop'

## Goals / Non-Goals

**Non-Goals:**
- een apart instellingenvenster
- sneltoets en meldingen, die komen in change 5 en 6
- het menubalk-icoon verbergen

## Decisions

- **Vinkje in het bestaande menu, geen instellingenvenster.** Voor één aan/uit-keuze is een venster veel code voor weinig. Een `CheckMenuItem` laat de status zelf zien.
- **Island verbergen, niet weggooien.** Het venster blijft bestaan en blijft het `now-playing` event krijgen. Daardoor klopt het nummer meteen als je het weer aanzet. Alternatief was het venster sluiten en opnieuw aanmaken, maar dan moet de notch-positie opnieuw.
- **Eigen `settings.json` in de app-configmap, geen `tauri-plugin-store`.** Het is één boolean en `serde_json` zit er al in. De plugin is een nieuwe dependency en een extra capability voor bijna niets.
- **Rust leest en schrijft het bestand.** De keuze wordt in het menu gemaakt en het menu leeft in Rust, dus de webview hoeft er niets van te weten.
- **Bij lezen valt alles terug op aan.** Geen bestand, kapot bestand of een onbekend veld: standaard aan, de app start gewoon.
- **Start bij inloggen via `tauri-plugin-autostart`, niet in `settings.json`.** macOS houdt zelf bij of de app bij inloggen start, dus de plugin leest de status daar uit. Dan kan het vinkje niet uit de pas lopen met een eigen kopie. Alleen vanuit Rust gebruikt, dus geen nieuwe capability voor de webview.
- **`MacosLauncher::LaunchAgent`.** Zet een LaunchAgent in `~/Library/LaunchAgents`. Alternatief was de AppleScript-launcher, maar die vraagt om toestemming om System Events te bedienen.

## Risks / Trade-offs

- [Schrijven mislukt, bijvoorbeeld een volle schijf] → het island gaat wel aan of uit, alleen de keuze is na een herstart weg
- [In `pnpm tauri dev` wijst de LaunchAgent naar de debug-build] → start bij inloggen testen met de gebouwde `.app`, en de LaunchAgent na het testen weer uitzetten
- [Uitzetten bij Systeeminstellingen laat de LaunchAgent vaak staan, dus het vinkje kan aan blijven] → geaccepteerd, uit- en weer aanvinken in het menu zet het recht
- [Het vinkje toggelt zelf bij een klik] → na de klik de status van het vinkje uitlezen, niet zelf omdraaien, anders lopen vinkje en island uit elkaar
