## MODIFIED Requirements

### Requirement: Active pane visual indicator

The currently focused pane SHALL be visually distinguished from inactive panes.

#### Scenario: Focused pane is highlighted
- **WHEN** a pane has focus
- **THEN** its border SHALL be rendered in `Color::Yellow` compared to inactive panes which use `Color::Blue`
