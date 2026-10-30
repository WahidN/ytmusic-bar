## Purpose

Geeft je één vaste plek in de menubalk om YouTube Music te openen, te gebruiken en ingelogd te blijven.

## ADDED Requirements

### Requirement: Icoon in de menubalk
De app MUST een icoon in de menubalk tonen en MUST NOT in het Dock of de app-switcher verschijnen.

#### Scenario: App start
- **WHEN** je de app start
- **THEN** staat er een icoon in de menubalk
- **AND** staat er geen icoon in het Dock

### Requirement: Popover met YouTube Music
Een klik op het icoon MUST een venster direct onder het icoon openen of sluiten. Het venster MUST music.youtube.com tonen met alle functies van de site, zoals zoeken, playlists en de spelerbalk.

#### Scenario: Popover openen
- **WHEN** de popover dicht is en je klikt op het icoon
- **THEN** opent de popover onder het icoon met YouTube Music

#### Scenario: Popover sluiten met het icoon
- **WHEN** de popover open is en je klikt op het icoon
- **THEN** gaat de popover dicht

#### Scenario: Popover sluiten met een klik ernaast
- **WHEN** de popover open is en je klikt ergens anders
- **THEN** gaat de popover dicht

### Requirement: Doorspelen op de achtergrond
Muziek MUST blijven spelen als de popover dicht is. De pagina MUST NOT opnieuw laden bij het openen van de popover.

#### Scenario: Popover sluiten tijdens het afspelen
- **WHEN** er muziek speelt en de popover gaat dicht
- **THEN** speelt de muziek door
- **AND** staat het nummer op dezelfde plek als je de popover weer opent

### Requirement: Login blijft bewaard
Na het inloggen met een Google-account MUST de login bewaard blijven na het sluiten en herstarten van de app.

#### Scenario: Herstart na inloggen
- **WHEN** je bent ingelogd, de app stopt en je de app opnieuw start
- **THEN** ben je nog steeds ingelogd in YouTube Music

#### Scenario: Google accepteert de login
- **WHEN** je inlogt via de knop 'Inloggen' in YouTube Music
- **THEN** toont Google het normale inlogscherm en geen melding dat de browser niet veilig is

### Requirement: App stoppen
Een rechtsklik op het icoon MUST een menu tonen met 'Stop'. 'Stop' MUST de app afsluiten.

#### Scenario: Stoppen via het menu
- **WHEN** je met rechts op het icoon klikt en 'Stop' kiest
- **THEN** stopt de app en verdwijnt het icoon
