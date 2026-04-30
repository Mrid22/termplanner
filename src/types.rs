use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct DateEntry {
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

#[derive(Debug, Clone)]
pub struct NoteEntry {
    pub filename: String,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct DateState {
    pub items: Vec<DateEntry>,
    pub list_state: ListState,
}

#[derive(Debug, Default)]
pub struct TaskState {
    pub items: Vec<Task>,
    pub list_state: ListState,
    pub is_adding: bool,
    pub task_input_val: String,
}

#[derive(Debug, Default)]
pub struct NoteState {
    pub items: Vec<NoteEntry>,
    pub list_state: ListState,
    pub is_adding: bool,
    pub note_input_val: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivePane {
    #[default]
    Dates,
    Tasks,
    Notes,
}

impl ActivePane {
    pub fn next(self) -> Self {
        match self {
            ActivePane::Dates => ActivePane::Tasks,
            ActivePane::Tasks => ActivePane::Notes,
            ActivePane::Notes => ActivePane::Dates,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            ActivePane::Dates => ActivePane::Notes,
            ActivePane::Tasks => ActivePane::Dates,
            ActivePane::Notes => ActivePane::Tasks,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub date_state: DateState,
    pub task_state: TaskState,
    pub note_state: NoteState,
    pub active_pane: ActivePane,
    pub selected_date: Option<String>,
}
