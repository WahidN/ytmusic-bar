## Why

YouTube Music heeft geen Mac-app. Een browsertab raak je kwijt tussen andere tabs en je moet steeds opnieuw zoeken waar de muziek speelt. Een icoon in de menubalk geeft één vaste plek om muziek te kiezen en te bedienen.

## What Changes

- nieuw Tauri v2 project met React en TypeScript
- icoon in de menubalk, geen icoon in het Dock
- klik op het icoon opent een popover-venster met de volledige YouTube Music site
- de popover sluit bij een klik ernaast, de muziek blijft spelen
- login blijft bewaard na het herstarten van de app
- rechtsklik op het icoon toont een menu met 'Stop'

Dit is change 1 van 4. De volgende changes:

1. `add-popover-player`: deze change
2. `add-now-playing-bridge`: nummer, artiest, cover en play/pause/vorige/volgende uitlezen en bedienen vanuit de app
3. `add-notch-island`: Dynamic Island-paneel bij de notch dat groter wordt bij hover
4. `add-settings`: instelling om te kiezen tussen menubalk en notch

## Capabilities

### New Capabilities
- `popover-player`: menubalk-icoon, popover met YouTube Music, bewaarde login en doorspelen op de achtergrond

### Modified Capabilities

## Impact

- nieuw project, er is nog geen code
- Rust toolchain nodig (via rustup)
- dependencies: `tauri` v2, `tauri-plugin-positioner`, React, Vite
