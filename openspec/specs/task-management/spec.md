# task-management Specification

## Purpose
TBD - created by archiving change terminal-notes-tasks-app. Update Purpose after archive.
## Requirements
### Requirement: Task loading from tasks.md

The application SHALL load tasks from the selected date's `tasks.md` file, parsing the markdown checkbox format.

#### Scenario: Load unchecked tasks
- **WHEN** `tasks.md` contains `- [ ] Buy groceries`
- **THEN** the Tasks pane SHALL display "Buy groceries" as an unchecked task

#### Scenario: Load checked tasks
- **WHEN** `tasks.md` contains `- [x] Buy groceries`
- **THEN** the Tasks pane SHALL display "Buy groceries" as a checked (completed) task

#### Scenario: Mixed tasks
- **WHEN** `tasks.md` contains both checked and unchecked tasks
- **THEN** all tasks SHALL be displayed with their correct completion status

#### Scenario: Malformed lines are skipped
- **WHEN** `tasks.md` contains lines not matching `- [ ]` or `- [x]` format
- **THEN** those lines SHALL be skipped without crashing

### Requirement: Task saving to tasks.md

Changes to tasks (add, delete, toggle) SHALL be persisted to `tasks.md` immediately.

#### Scenario: Toggle task saves immediately
- **WHEN** the user toggles a task's completion status
- **THEN** `tasks.md` SHALL be updated with the new `- [x]` or `- [ ]` format

#### Scenario: Add task creates file if needed
- **WHEN** the user adds a task and `tasks.md` does not exist
- **THEN** `tasks.md` SHALL be created and the task SHALL be written

### Requirement: Add new task

The user SHALL be able to add a new task to the selected date using the `A` key.

#### Scenario: Enter add mode
- **WHEN** the Tasks pane is focused and user presses `A`
- **THEN** a text input popup SHALL appear for entering the task description

#### Scenario: Submit new task
- **WHEN** the user types a description and presses Enter in add mode
- **THEN** the task SHALL be added to the list and saved to `tasks.md`

#### Scenario: Cancel add task
- **WHEN** the user presses Escape in add mode
- **THEN** add mode SHALL exit without adding a task

### Requirement: Delete task

The user SHALL be able to delete the selected task using the `D` key.

#### Scenario: Delete selected task
- **WHEN** a task is selected and user presses `D`
- **THEN** the task SHALL be removed from the list and from `tasks.md`

### Requirement: Toggle task completion

The user SHALL be able to toggle a task's completion status using Enter or Space.

#### Scenario: Toggle unchecked to checked
- **WHEN** an unchecked task is selected and user presses Enter
- **THEN** the task SHALL be marked as checked and saved to `tasks.md`

#### Scenario: Toggle checked to unchecked
- **WHEN** a checked task is selected and user presses Space
- **THEN** the task SHALL be marked as unchecked and saved to `tasks.md`

