# clean-task-display Specification

## Purpose
Tasks in the Tasks pane are rendered without markdown checkbox markers, with completed tasks showing strikethrough styling.

## Requirements

### Requirement: Hide markdown checkbox markers in UI

The Tasks pane SHALL NOT display raw markdown checkbox markers (`[ ]` or `[x]`) when rendering tasks. Only the task description text SHALL be visible.

#### Scenario: Unchecked task displays without marker
- **WHEN** a task has `done: false`
- **THEN** the Tasks pane SHALL display only the task description without `[ ]` prefix

#### Scenario: Checked task displays without marker
- **WHEN** a task has `done: true`
- **THEN** the Tasks pane SHALL display only the task description without `[x]` prefix

### Requirement: Strikethrough styling for completed tasks

The Tasks pane SHALL apply strikethrough (crossed-out) text styling to tasks that are marked as completed.

#### Scenario: Completed task shows strikethrough
- **WHEN** a task is marked as done
- **THEN** the task description SHALL be rendered with strikethrough styling in the Tasks pane

#### Scenario: Incomplete task has no strikethrough
- **WHEN** a task is not marked as done
- **THEN** the task description SHALL be rendered without strikethrough styling
