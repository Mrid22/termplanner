## ADDED Requirements

### Requirement: Note listing per date

The application SHALL list all `.md` files from the selected date's `Notes/` subdirectory in the Notes pane.

#### Scenario: Load notes from directory
- **WHEN** a date is selected and `~/Notes/YYYY-MM-DD/Notes/` contains `.md` files
- **THEN** the Notes pane SHALL display each file name (without `.md` extension) as a note entry

#### Scenario: No notes directory
- **WHEN** the selected date has no `Notes/` subdirectory
- **THEN** the Notes pane SHALL display an empty list

#### Scenario: Empty notes directory
- **WHEN** the `Notes/` subdirectory exists but contains no `.md` files
- **THEN** the Notes pane SHALL display an empty list

### Requirement: Open note in external editor

Selecting a note SHALL open the corresponding `.md` file in the user's `$EDITOR`.

#### Scenario: Open note with EDITOR set
- **WHEN** a note is selected and user presses Enter
- **THEN** the application SHALL launch `$EDITOR` with the note's file path

#### Scenario: Open note with EDITOR not set
- **WHEN** `$EDITOR` is not set and user selects a note
- **THEN** the application SHALL attempt to open with `vi` as fallback, or display an error if neither is available

#### Scenario: Editor returns to app
- **WHEN** the external editor is closed
- **THEN** control SHALL return to the terminal application

### Requirement: Add new note

The user SHALL be able to create a new note for the selected date.

#### Scenario: Create new note
- **WHEN** the Notes pane is focused and user presses `A`
- **THEN** a text input prompt SHALL appear for the note name, and a new `.md` file SHALL be created and opened in `$EDITOR`

#### Scenario: Note name sanitization
- **WHEN** the user enters a note name with spaces or special characters
- **THEN** the filename SHALL be sanitized to a safe format (e.g., spaces replaced with hyphens)
