## 1. Venster over de notch

- [x] 1.1 `macos-private-api` aan en een `island` venster (transparant, geen rand, focusbaar) in de config, check dat de app start zonder fouten
- [x] 1.2 Notch-maat uitlezen met `NSScreen` en het venster daar neerzetten met het juiste window level en collection behavior, check met een tijdelijke rode achtergrond dat het venster precies over de notch ligt, ook in fullscreen
- [x] 1.3 Geen notch: geen island, check door de notch-check tijdelijk om te draaien

## 2. Data en bediening

- [x] 2.1 Commands `get_now_playing`, `media_action` en `set_island_size` plus capability voor `island`, check dat de YouTube Music pagina ze niet kan aanroepen
- [x] 2.2 React-template vervangen door een island-component dat state ophaalt en naar `now-playing` luistert, check dat titel en cover na een nummerwissel kloppen

## 3. Island UI

- [x] 3.1 Compacte weergave: zwart bij pauze, breder met cover en balkjes tijdens afspelen, check met afspelen en pauzeren
- [x] 3.2 Klik: venster groot, vorm groeit uit tot mini player met knoppen, check klik en dat de knoppen werken
- [x] 3.3 Klik ernaast: vorm krimpt, daarna venster klein, check dat de menubalk naast het island klikbaar blijft
- [x] 3.4 `prefers-reduced-motion` respecteren, check met de instelling aan

## 4. Controle

- [x] 4.1 `cargo clippy`, `pnpm tsc --noEmit` en `pnpm tauri build` zonder fouten
