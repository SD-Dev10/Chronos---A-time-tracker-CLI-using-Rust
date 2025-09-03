use std::collections::HashMap;
use std::time::{Duration, Instant};
use tui_textarea::TextArea;
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub status: TaskStatus,
    pub time_spent: Duration,
    pub started_at: Option<Instant>,
    pub expected_duration: Duration,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Active,
    Paused,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub next_id: u32,
    pub textarea: TextArea<'static>,
    pub focus_textarea: bool,
    pub selected_index: Option<usize>,
    pub show_popup: bool,
    pub show_green_log: bool,
    pub green_log_task: Option<String>,
    pub breaks: HashMap<u32, Break>,
}
impl App {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            textarea: TextArea::default(),
            focus_textarea: true,
            selected_index: None,
            show_popup: false,
            show_green_log: false,
            green_log_task: None,
            breaks: HashMap::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BreakStatus {
    Started,
    Ongoing,
    Completed,
    Exceeded,
}

pub struct Break {
    pub break_status: BreakStatus,
    pub break1_started_at: Option<Instant>,
    pub break2_started_at: Option<Instant>,
    pub break3_started_at: Option<Instant>,
    pub break_interval_1: Duration,
    pub break_interval_2: Duration,
    pub break_interval_3: Duration,
    pub elapsed_time_1: Duration,
    pub elapsed_time_2: Duration,
    pub elapsed_time_3: Duration,
}
impl Break {
    pub fn new() -> Self {
        Self {
            break_status: BreakStatus::Started,
            break1_started_at: Some(Instant::now()),
            break2_started_at: None,
            break3_started_at: None,
            break_interval_1: Duration::new(0, 0),
            break_interval_2: Duration::new(0, 0),
            break_interval_3: Duration::new(0, 0),
            elapsed_time_1: Duration::new(0, 0),
            elapsed_time_2: Duration::new(0, 0),
            elapsed_time_3: Duration::new(0, 0),
        }
    }
}
