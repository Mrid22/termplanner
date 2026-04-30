# file-storage Specification

## Purpose
TBD - created by archiving change terminal-notes-tasks-app. Update Purpose after archive.
## Requirements
### Requirement: File storage directory structure

The application SHALL store all data under `~/Notes/` with a date-based folder hierarchy. Each date folder SHALL follow the `YYYY-MM-DD` naming convention.

#### Scenario: Directory structure on first run
- **WHEN** the application starts and `~/Notes/` does not exist
- **THEN** the application SHALL create `~/Notes/` directory

#### Scenario: Date directory discovery
- **WHEN** the application scans `~/Notes/`
- **THEN** it SHALL identify all subdirectories matching the `YYYY-MM-DD` format

#### Scenario: Notes subdirectory
- **WHEN** a date directory exists (e.g., `~/Notes/2026-04-30/`)
- **THEN** notes SHALL be stored in `~/Notes/2026-04-30/Notes/` subdirectory

### Requirement: Tasks file format

Tasks for each date SHALL be stored in `tasks.md` within the corresponding date directory.

#### Scenario: Task file location
- **WHEN** loading tasks for date `2026-04-30`
- **THEN** the application SHALL read from `~/Notes/2026-04-30/tasks.md`

#### Scenario: Task file creation
- **WHEN** a task is added for a date that has no `tasks.md`
- **THEN** the application SHALL create `tasks.md` in that date's directory

### Requirement: Notes file format

Each note SHALL be stored as an individual `.md` file within the date's `Notes/` subdirectory.

#### Scenario: Note file location
- **WHEN** loading notes for date `2026-04-30`
- **THEN** the application SHALL list all `.md` files in `~/Notes/2026-04-30/Notes/`

#### Scenario: Note file creation
- **WHEN** a new note is created for a date
- **THEN** a new `.md` file SHALL be created in the date's `Notes/` subdirectory

