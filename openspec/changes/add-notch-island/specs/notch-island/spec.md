## Purpose

Toont het huidige nummer en de bediening in een Dynamic Island-paneel over de notch van een MacBook.

## ADDED Requirements

### Requirement: Island over de notch
Op een scherm met notch MUST de app een zwart paneel tonen dat de notch precies bedekt. Het paneel MUST op elke Space en boven apps in fullscreen zichtbaar zijn. Op een Mac zonder scherm met notch MUST de app geen paneel tonen.

#### Scenario: Niets aan het spelen
- **WHEN** er geen muziek speelt
- **THEN** is het paneel even groot als de notch en valt het weg tegen de notch

#### Scenario: Geen notch
- **WHEN** de app start op een Mac zonder scherm met notch
- **THEN** is er geen paneel
- **AND** werken het menubalk-icoon en de popover zoals voorheen

### Requirement: Compacte weergave tijdens afspelen
Als er muziek speelt MUST het paneel breder worden dan de notch, met links een kleine cover en rechts bewegende balkjes. Bij pauze MUST het paneel terug naar de breedte van de notch.

#### Scenario: Muziek start
- **WHEN** er een nummer begint te spelen
- **THEN** wordt het paneel breder met de cover links en de balkjes rechts

#### Scenario: Muziek pauzeert
- **WHEN** de muziek pauzeert
- **THEN** krimpt het paneel terug tot de breedte van de notch

### Requirement: Mini player bij klik
Als je op het paneel klikt MUST het paneel uitgroeien tot een mini player met cover, titel, artiest en de knoppen vorige, afspelen of pauzeren, en volgende. Als je ernaast klikt MUST het paneel terug naar de vorige weergave. De knoppen MUST hetzelfde doen als de knoppen in YouTube Music.

#### Scenario: Klik
- **WHEN** je op het paneel klikt
- **THEN** groeit het paneel uit tot de mini player met het huidige nummer

#### Scenario: Volgende in de mini player
- **WHEN** je in de mini player op volgende klikt
- **THEN** start het volgende nummer
- **AND** toont de mini player het nieuwe nummer binnen 2 seconden

#### Scenario: Klik ernaast
- **WHEN** je ergens anders klikt
- **THEN** krimpt het paneel terug

### Requirement: Menubalk blijft bruikbaar
Het paneel MUST NOT klikken op de menubalk blokkeren buiten de zichtbare vorm van het paneel.

#### Scenario: Klik naast het paneel
- **WHEN** je klikt op een menubalk-item naast het paneel
- **THEN** reageert dat item gewoon
