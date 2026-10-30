## Purpose

Laat je de popover openen en sluiten met een vaste sneltoets vanuit elke app, en die sneltoets aan- en uitzetten vanuit het menu.

## ADDED Requirements

### Requirement: Popover met sneltoets
De sneltoets ⌃⌥⌘M MUST de popover openen onder het menubalk-icoon als hij dicht is, en sluiten als hij open is. Dit MUST werken vanuit elke app, ook als ytmusic-bar geen focus heeft. Ingedrukt houden MUST NOT de popover steeds open en dicht laten gaan.

#### Scenario: Openen vanuit een andere app
- **WHEN** je in een andere app werkt en ⌃⌥⌘M drukt
- **THEN** opent de popover onder het menubalk-icoon met focus

#### Scenario: Sluiten
- **WHEN** de popover open is en je ⌃⌥⌘M drukt
- **THEN** gaat de popover dicht en speelt de muziek door

### Requirement: Sneltoets aan- en uitzetten
Het rechtsklik-menu MUST een vinkje 'Sneltoets ⌃ ⌥ ⌘ M' hebben. Vinkje uit MUST de sneltoets meteen vrijgeven, zodat andere apps hem weer krijgen. Vinkje aan MUST hem meteen weer laten werken. Zonder eerdere keuze MUST het vinkje aan staan. De keuze MUST bewaard blijven na een herstart.

#### Scenario: Uitzetten
- **WHEN** je 'Sneltoets ⌃ ⌥ ⌘ M' uitvinkt en ⌃⌥⌘M drukt
- **THEN** gebeurt er niets in ytmusic-bar

#### Scenario: Herstart met sneltoets uit
- **WHEN** je de sneltoets uitzet en de app opnieuw start
- **THEN** staat het vinkje uit en doet ⌃⌥⌘M niets in ytmusic-bar

### Requirement: Sneltoets bezet
Als de app de sneltoets niet kan registreren, MUST het vinkje uit staan en MUST de rest van de app gewoon werken.

#### Scenario: Registreren mislukt
- **WHEN** ⌃⌥⌘M bij het starten niet te registreren is
- **THEN** start de app zonder fout
- **AND** staat 'Sneltoets ⌃ ⌥ ⌘ M' uit
