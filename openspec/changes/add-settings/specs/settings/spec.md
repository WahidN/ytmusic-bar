## Purpose

Laat je het island over de notch en het starten bij inloggen aan- en uitzetten vanuit het menu van het menubalk-icoon, en onthoudt die keuzes tussen herstarts.

## ADDED Requirements

### Requirement: Island aan- en uitzetten
Het rechtsklik-menu van het menubalk-icoon MUST een vinkje 'Toon in notch' hebben. Vinkje uit MUST het island meteen verbergen. Vinkje aan MUST het island meteen weer tonen, met het huidige nummer. Het menubalk-icoon MUST altijd zichtbaar blijven, ook als het island uit staat.

#### Scenario: Island uitzetten
- **WHEN** je 'Toon in notch' uitvinkt
- **THEN** verdwijnt het island van de notch
- **AND** werken het menubalk-icoon, de popover en de bediening in het menu gewoon

#### Scenario: Island aanzetten
- **WHEN** je 'Toon in notch' weer aanvinkt terwijl er muziek speelt
- **THEN** staat het island meteen weer op de notch, breder met cover en balkjes

### Requirement: Keuze blijft bewaard
De app MUST de keuze voor 'Toon in notch' bewaren en bij de volgende start gebruiken. Zonder bewaarde keuze MUST het vinkje aan staan. Een kapot of onleesbaar bestand MUST NOT de app laten crashen, dan geldt de standaard.

#### Scenario: Herstart met island uit
- **WHEN** je 'Toon in notch' uitvinkt en de app opnieuw start
- **THEN** is er geen island
- **AND** staat het vinkje uit

#### Scenario: Eerste start
- **WHEN** de app voor het eerst start op een MacBook met notch
- **THEN** staat het vinkje aan en is het island zichtbaar

### Requirement: Geen notch
Op een Mac zonder scherm met notch MUST het vinkje 'Toon in notch' grijs zijn en niets doen.

#### Scenario: Klep dicht met een extern scherm
- **WHEN** de app start zonder scherm met notch
- **THEN** is 'Toon in notch' grijs en niet aan te klikken

### Requirement: Start bij inloggen
Het rechtsklik-menu MUST een vinkje 'Start bij inloggen' hebben. Vinkje aan MUST de app laten starten als je inlogt op de Mac. Vinkje uit MUST dat stoppen. Zonder eerdere keuze MUST het vinkje uit staan.

#### Scenario: Aanzetten
- **WHEN** je 'Start bij inloggen' aanvinkt en daarna uit- en weer inlogt op de Mac
- **THEN** start de app vanzelf met het icoon in de menubalk

#### Scenario: Uitzetten
- **WHEN** je 'Start bij inloggen' uitvinkt en daarna uit- en weer inlogt
- **THEN** start de app niet vanzelf
