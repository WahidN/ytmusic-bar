## Purpose

Laat de app weten wat er in YouTube Music speelt en laat je afspelen bedienen zonder de popover te openen.

## ADDED Requirements

### Requirement: Huidig nummer uitlezen
De app MUST titel, artiest, cover-URL en speel-status van het huidige nummer bijhouden. Een wissel van nummer of van spelen naar pauze MUST binnen 2 seconden zichtbaar zijn in de app, ook als de popover dicht is.

#### Scenario: Nieuw nummer start
- **WHEN** er een ander nummer begint
- **THEN** toont de tooltip van het icoon de nieuwe titel en artiest binnen 2 seconden

#### Scenario: Niets gespeeld
- **WHEN** er sinds de start van de app nog niets gespeeld heeft
- **THEN** toont de tooltip 'YouTube Music'

### Requirement: Afspelen bedienen
Het rechtsklik-menu van het icoon MUST 'Vorige', 'Afspelen' of 'Pauzeren', en 'Volgende' tonen. Elke actie MUST hetzelfde doen als de knop in de spelerbalk van YouTube Music.

#### Scenario: Pauzeren via het menu
- **WHEN** er muziek speelt en je kiest 'Pauzeren'
- **THEN** stopt de muziek
- **AND** toont het menu daarna 'Afspelen'

#### Scenario: Volgende via het menu
- **WHEN** je 'Volgende' kiest
- **THEN** start het volgende nummer uit de wachtrij

#### Scenario: Bediening met popover dicht
- **WHEN** de popover dicht is en je kiest een actie in het menu
- **THEN** voert YouTube Music de actie uit zonder dat de popover opent

### Requirement: Beperkte toegang voor de pagina
De YouTube Music pagina MUST alleen nummer-info naar de app kunnen sturen. De pagina MUST NOT andere commands of plugins van de app kunnen aanroepen.

#### Scenario: Pagina roept een ander command aan
- **WHEN** een script op music.youtube.com een ander command dan het nummer-info command aanroept
- **THEN** weigert de app de aanroep
