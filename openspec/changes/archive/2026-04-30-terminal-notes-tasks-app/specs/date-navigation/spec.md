## ADDED Requirements

### Requirement: Date scanning and listing

The application SHALL scan `~/Notes/` for directories matching `YYYY-MM-DD` format and display them in the Dates pane, sorted in chronological order.

#### Scenario: Load dates from filesystem
- **WHEN** the application starts
- **THEN** it SHALL scan `~/Notes/` and populate the Dates pane with all matching directories sorted oldest to newest

#### Scenario: No dates exist
- **WHEN** `~/Notes/` is empty or does not exist
- **THEN** the Dates pane SHALL display an empty list

#### Scenario: Invalid directory names are ignored
- **WHEN** `~/Notes/` contains directories not matching `YYYY-MM-DD`
- **THEN** those directories SHALL be excluded from the Dates pane

### Requirement: Date selection updates other panes

Selecting a date in the Dates pane SHALL update the Tasks and Notes panes to show content for that date.

#### Scenario: Select a date with content
- **WHEN** the user selects a date that has tasks and notes
- **THEN** the Tasks pane SHALL display tasks from that date's `tasks.md` and the Notes pane SHALL list notes from that date's `Notes/` directory

#### Scenario: Select a date with no content
- **WHEN** the user selects a date that has no `tasks.md` and no notes
- **THEN** both the Tasks and Notes panes SHALL display empty lists

#### Scenario: No date selected
- **WHEN** no date is selected (empty date list)
- **THEN** the Tasks and Notes panes SHALL display empty lists
