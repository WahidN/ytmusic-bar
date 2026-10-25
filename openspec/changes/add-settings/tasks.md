## 1. Keuze bewaren

- [x] 1.1 `settings.json` lezen en schrijven in de app-configmap, standaard aan bij geen of kapot bestand, check door het bestand te wissen en met onzin te vullen en de app te starten

## 2. Vinkje in het menu

- [ ] 2.1 `CheckMenuItem` 'Toon in notch' tussen de bediening en 'Stop', met de bewaarde keuze als begin-status, check dat het menu het vinkje toont
- [ ] 2.2 Klik op het vinkje: island verbergen of tonen en de keuze opslaan, check aan en uit tijdens afspelen en dat het nummer klopt na weer aanzetten
- [ ] 2.3 Bij start het island alleen tonen als de keuze aan staat, check door uit te zetten en de app opnieuw te starten
- [x] 2.4 Zonder notch is het vinkje grijs, check met de klep dicht en een extern scherm

## 3. Start bij inloggen

- [x] 3.1 `tauri-plugin-autostart` toevoegen met `MacosLauncher::LaunchAgent`, check dat de app start zonder fouten
- [x] 3.2 `CheckMenuItem` 'Start bij inloggen' onder 'Toon in notch', begin-status uit de plugin, klik zet het aan of uit, check dat de LaunchAgent in `~/Library/LaunchAgents` verschijnt en verdwijnt
- [ ] 3.3 Met de gebouwde `.app` aanzetten, uit- en weer inloggen, check dat de app vanzelf start, daarna weer uitzetten

## 4. Controle

- [x] 4.1 `cargo clippy`, `pnpm tsc --noEmit` en `pnpm tauri build` zonder fouten
