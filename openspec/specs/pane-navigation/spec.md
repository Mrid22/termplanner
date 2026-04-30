# pane-navigation Specification

## Purpose
TBD - created by archiving change terminal-notes-tasks-app. Update Purpose after archive.
## Requirements
### Requirement: Pane focus cycling with Tab

The user SHALL cycle focus between the three panes (Dates, Tasks, Notes) using Tab (forward) and Shift+Tab (backward).

#### Scenario: Tab cycles forward
- **WHEN** the Dates pane is focused and user presses Tab
- **THEN** focus SHALL move to the Tasks pane

#### Scenario: Tab cycles from last to first
- **WHEN** the Notes pane is focused and user presses Tab
- **THEN** focus SHALL cycle back to the Dates pane

#### Scenario: Shift+Tab cycles backward
- **WHEN** the Tasks pane is focused and user presses Shift+Tab
- **THEN** focus SHALL move to the Dates pane

#### Scenario: Shift+Tab cycles from first to last
- **WHEN** the Dates pane is focused and user presses Shift+Tab
- **THEN** focus SHALL cycle to the Notes pane

### Requirement: Active pane visual indicator

The currently focused pane SHALL be visually distinguished from inactive panes.

#### Scenario: Focused pane is highlighted
- **WHEN** a pane has focus
- **THEN** its border SHALL be rendered in `Color::Yellow` compared to inactive panes which use `Color::Blue`

### Requirement: Input scoped to active pane

Keyboard navigation input (j/k/Enter/Space/D/A) SHALL only affect the currently focused pane.

#### Scenario: Navigation in Tasks pane
- **WHEN** the Tasks pane is focused and user presses j or k
- **THEN** the selection SHALL move within the Tasks list only

#### Scenario: Navigation in Dates pane does not affect Tasks
- **WHEN** the Dates pane is focused and user presses j or k
- **THEN** the selection SHALL move within the Dates list and the Tasks list selection SHALL remain unchanged

#### Scenario: Quit works from any pane
- **WHEN** user presses q or Escape from any focused pane
- **THEN** the application SHALL quit

