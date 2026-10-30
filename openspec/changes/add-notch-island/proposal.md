## Why

Op een MacBook met notch is de ruimte rond de notch leeg. Een Dynamic Island daar laat zien wat er speelt en geeft bediening zonder dat je de popover hoeft te openen.

## What Changes

- nieuw venster dat precies over de notch ligt, zwart, dus onzichtbaar als er niets speelt
- tijdens het afspelen wordt de pill iets breder: kleine cover links, bewegende balkjes rechts
- klik op de pill: hij groeit uit tot een mini player met cover, titel, artiest en vorige, afspelen/pauzeren en volgende
- klik ernaast: de pill krimpt terug
- geen notch op het scherm: geen island, de rest van de app werkt gewoon
- het menubalk-icoon blijft, kiezen tussen icoon en notch komt in change 4

## Capabilities

### New Capabilities
- `notch-island`: Dynamic Island-paneel over de notch met nummer-info en bediening

### Modified Capabilities

## Impact

- `src-tauri/src/lib.rs` en een nieuwe module voor de notch-positie (macOS API via `objc2-app-kit`)
- `src/`: de React-template wordt vervangen door de island-UI
- nieuwe capability voor het island-venster met twee commands en het `now-playing` event
- `macos-private-api` aan in Tauri, nodig voor een transparant venster. Gevolg: de app kan niet in de App Store
