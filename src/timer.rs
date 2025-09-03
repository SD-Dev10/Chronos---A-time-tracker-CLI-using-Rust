use crate::redundancy_warning::pop_up;
use crate::task_ui::run;
use crate::util::{ App, Break, BreakStatus, Task, TaskStatus };

use color_eyre::Result;

use crossterm::event::{ self, Event, KeyCode, KeyEventKind };

use ratatui::prelude::*;

use ratatui::text::Span;
use ratatui::widgets::{
    Block,
    BorderType,
    Borders,
    Cell,
    Gauge,
    List,
    ListItem,
    Padding,
    Row,
    Table,
};
use ratatui::{ style::{ Color, Modifier, Style }, DefaultTerminal };

use std::time::{ Duration, Instant };
use tui_textarea::TextArea;

pub fn run_timer(
    mut terminal: DefaultTerminal,
    mut app: App
) -> Result<(), Box<dyn std::error::Error>> {
    let mut break_inst = Break::new();
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(100)])
                .split(frame.area());

            frame.render_widget(
                Block::default().style(Style::default().bg(Color::Rgb(10, 14, 32))),
                layout[0]
            );

            let task_top = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    vec![
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                        Constraint::Percentage(70),
                        Constraint::Percentage(10)
                    ]
                )
                .split(layout[0]);

            // HEADER (Not focusable)
            let header_cells = vec!["[Welcome to Chronos]", "[Mode: Timer]"];
            let header = Row::new(
                header_cells
                    .iter()
                    .map(|h| Cell::from(*h))
                    .collect::<Vec<_>>()
            ).style(Style::new().fg(Color::Rgb(102, 217, 239)));

            let header_widths = [Constraint::Percentage(90), Constraint::Percentage(10)];
            frame.render_widget(
                Table::new(Vec::<Row>::new(), header_widths)
                    .header(header)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .block(
                        Block::new()
                            .title(
                                Span::styled(
                                    "Chronos",
                                    Style::new()
                                        .fg(Color::Rgb(255, 165, 0)) // change title color
                                        .add_modifier(Modifier::BOLD)
                                )
                            )
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Plain)
                            .padding(Padding::new(1, 1, 1, 1))
                    ),
                task_top[0]
            );

            //NAVIGATION TABS (Focusable row 0)
            let task_layout = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(task_top[1]);

            app.textarea.set_block(
                Block::new()
                    .title("Resume TASK")
                    .title_alignment(Alignment::Center)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0))
            );
            frame.render_widget(&app.textarea, task_layout[0]);

            let nav_cells = vec!["[Tasks]", "|", "[Timer]"];
            let nav_cells_spans: Vec<Span> = nav_cells
                .iter()
                .map(|h| Span::raw(*h))
                .collect();

            let nav = Row::new(nav_cells_spans).style(Style::new().fg(Color::Rgb(255, 165, 0)));

            frame.render_widget(
                Table::new(Vec::<Row>::new(), [
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                ])
                    .header(nav)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .block(
                        Block::new()
                            .title("MODES")
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Plain)
                            .padding(Padding::new(1, 1, 1, 1))
                    ),
                task_layout[1]
            );
            // TASK TABLE HEADER
            let nested_task_data = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(task_top[2]);

            let nested_task_data_productivity = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
                .split(nested_task_data[1]);

            let task_cells = vec!["[ID]", "[Task Name]", "[Status]"]
                .iter()
                .map(|h| Cell::from(*h))
                .collect::<Vec<Cell>>();

            let task_header = Row::new(task_cells).style(Style::new().fg(Color::Rgb(255, 165, 0)));

            let task_rows: Vec<Row> = app.tasks
                .iter()
                .enumerate()
                .map(|(i, task)| {
                    let status_str = match task.status {
                        TaskStatus::Active => "Active",
                        TaskStatus::Paused => "Paused",
                    };

                    let mut row = Row::new(
                        vec![
                            Cell::from(task.id.to_string()),
                            Cell::from(task.name.clone()),
                            Cell::from(status_str)
                        ]
                    );

                    if Some(i) == app.selected_index {
                        row = row.style(
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        );
                    }

                    row
                })
                .collect();

            let task_widths = [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ];

            frame.render_widget(
                Table::new(task_rows, task_widths)
                    .header(task_header)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .block(
                        Block::new()
                            .title("TASKS")
                            .borders(
                                Borders::TOP | Borders::LEFT | Borders::RIGHT | Borders::BOTTOM
                            )
                            .border_type(ratatui::widgets::BorderType::Plain)
                            .padding(Padding::new(1, 1, 1, 1))
                    ),
                nested_task_data[0]
            );

            if let Some(i) = app.selected_index {
                let task = app.tasks[i].clone();

                let elapsed = if let Some(started) = task.started_at {
                    task.time_spent + started.elapsed()
                } else {
                    task.time_spent
                };
                let progress_percentage = ((elapsed.as_secs_f64() / 7200.0) * 100.0).min(100.0);

                let outer = Block::default()
                    .borders(Borders::ALL)
                    .title("Task in progress")
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .padding(Padding::new(2, 2, 0, 0));

                let area = nested_task_data_productivity[1];

                frame.render_widget(outer.clone(), area);

                let inner = outer.inner(area);

                let gauge = Gauge::default()
                    .gauge_style(
                        Style::new()
                            .fg(Color::Rgb(30, 102, 245)) // filled (blue)
                            .on_light_cyan() // unfilled bg
                            .italic()
                            .add_modifier(Modifier::BOLD)
                    )
                    .percent(progress_percentage as u16)
                    .label(format!("{} {:.1}%", task.name, progress_percentage));

                frame.render_widget(gauge, inner);

                if matches!(task.status, TaskStatus::Paused) {
                    let mut filled = (
                        (progress_percentage / 100.0) *
                        (inner.width as f64)
                    ).round() as u16;

                    if filled > inner.width {
                        filled = inner.width;
                    }

                    //let strip_x = if filled == 0 { inner.x } else { inner.x + filled - 1 };

                    let red_strip_area = Rect {
                        x: inner.x + filled,
                        y: inner.y,
                        width: inner.width - filled,
                        height: inner.height,
                    };

                    let red_strip = Block::default().style(Style::default().bg(Color::Red));
                    frame.render_widget(red_strip, red_strip_area);
                }
            } else {
                let block_productivity_logs = Block::default()
                    .borders(Borders::ALL)
                    .title("Productivity_logs")
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)));
                frame.render_widget(block_productivity_logs, nested_task_data_productivity[0]);

                let block_progress = Block::default()
                    .borders(Borders::ALL)
                    .title("Task in progress")
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)));
                frame.render_widget(block_progress, nested_task_data_productivity[1]);
            }

            let break_logs: Vec<ListItem> = app.tasks
                .iter()
                .enumerate()
                .flat_map(|(i, t)| {
                    let mut items = Vec::new();
                    let five_minutes = Duration::from_secs(5 * 60);
                    let fifteen_minutes = Duration::from_secs(15 * 60);
                    let break_inst = app.breaks.entry(t.id).or_insert_with(Break::new);
                    if matches!(t.status, TaskStatus::Paused) {
                        let elapsed_break_1 = if let Some(started) = break_inst.break1_started_at {
                            started.elapsed().min(five_minutes)
                        } else {
                            break_inst.break_interval_1
                        };
                        let time_str_1 = {
                            let secs = elapsed_break_1.as_secs();
                            format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
                        };
                        let break_status_1 = if elapsed_break_1 == five_minutes {
                            BreakStatus::Completed
                        } else {
                            BreakStatus::Ongoing
                        };
                        let item = ListItem::new(
                            format!(
                                "Break_duration: {:?} task_name: {} first_break_state: {:?} first_break_status: {:?}",
                                time_str_1,
                                t.name,
                                BreakStatus::Started,
                                break_status_1
                            )
                        ).style(Style::new().fg(Color::Yellow));

                        items.push(item);

                        if
                            elapsed_break_1 == five_minutes &&
                            break_inst.break2_started_at.is_none()
                        {
                            break_inst.break2_started_at = Some(Instant::now());
                        }
                        break_inst.elapsed_time_1 = elapsed_break_1;
                    }
                    if
                        matches!(t.status, TaskStatus::Paused) &&
                        break_inst.elapsed_time_1 >= five_minutes
                    {
                        let elapsed_break_2 = if let Some(started) = break_inst.break2_started_at {
                            started.elapsed().min(five_minutes)
                        } else {
                            break_inst.break_interval_2
                        };
                        let time_str_2 = {
                            let secs = elapsed_break_2.as_secs();
                            format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
                        };
                        let break_status_2 = if elapsed_break_2 == five_minutes {
                            BreakStatus::Completed
                        } else {
                            BreakStatus::Ongoing
                        };
                        let item = ListItem::new(
                            format!(
                                "Break_duration: {:?} task_name: {} second_break_state: {:?} second_break_status: {:?}",
                                time_str_2,
                                t.name,
                                BreakStatus::Started,
                                break_status_2
                            )
                        ).style(Style::new().fg(Color::Yellow));

                        items.push(item);

                        if
                            elapsed_break_2 == five_minutes &&
                            break_inst.break3_started_at.is_none()
                        {
                            break_inst.break3_started_at = Some(Instant::now());
                        }
                        break_inst.elapsed_time_2 = elapsed_break_2;
                    }
                    if
                        matches!(t.status, TaskStatus::Paused) &&
                        break_inst.elapsed_time_1 >= five_minutes &&
                        break_inst.elapsed_time_2 >= five_minutes
                    {
                        let elapsed_break_3 = if let Some(started) = break_inst.break3_started_at {
                            started.elapsed().min(fifteen_minutes)
                        } else {
                            break_inst.break_interval_3
                        };

                        let time_str_3 = {
                            let secs = elapsed_break_3.as_secs();
                            format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
                        };

                        let break_status_3 = if elapsed_break_3 == fifteen_minutes {
                            BreakStatus::Completed
                        } else {
                            BreakStatus::Ongoing
                        };

                        let item = ListItem::new(
                            format!(
                                "Break_duration: {:?} task_name: {} final_break_state: {:?} final_break_status: {:?}",
                                time_str_3,
                                t.name,
                                BreakStatus::Started,
                                break_status_3
                            )
                        ).style(Style::new().fg(Color::Yellow));

                        items.push(item);
                        break_inst.elapsed_time_3 = elapsed_break_3;
                    }

                    if
                        matches!(t.status, TaskStatus::Paused) &&
                        break_inst.elapsed_time_1 >= five_minutes &&
                        break_inst.elapsed_time_2 >= five_minutes &&
                        break_inst.elapsed_time_3 >= fifteen_minutes
                    {
                        let item = ListItem::new(
                            format!(
                                "Break time exceeded for task: {}  final_break_status: {:?}",
                                t.name,
                                BreakStatus::Exceeded
                            )
                        )

                            .style(Style::new().fg(Color::Black).bg(Color::Red))
                            .add_modifier(Modifier::BOLD);
                        items.push(item);
                    }

                    if app.show_green_log && Some(t.name.clone()) == app.green_log_task {
                        let green_item = ListItem::new(
                            format!("Task: {} is activated successfully", t.name)
                        ).style(
                            Style::new()
                                .fg(Color::Black)
                                .bg(Color::Green)
                                .add_modifier(Modifier::BOLD)
                        );
                        items.push(green_item);
                    }

                    items
                })
                .collect();

            let break_list = List::new(break_logs)
                .block(
                    ratatui::widgets::Block
                        ::default()
                        .title("Productivity Logs")
                        .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                        .padding(Padding::new(1, 1, 2, 2))
                        .borders(ratatui::widgets::Borders::ALL)
                )

                .style(Style::default().fg(Color::White));
            frame.render_widget(break_list, nested_task_data_productivity[0]);

            let footer_cells = vec![
                "<Esc> Exit",
                "<UP/DOWN> Move",
                "<Tab> Focus",
                "<R> Resume Task",
                "<Delete> Delete Task"
            ];
            let footer_cells_spans: Vec<Span> = footer_cells
                .iter()
                .map(|h| Span::raw(*h))
                .collect();

            let footer = Row::new(footer_cells_spans).style(
                Style::new().fg(Color::Rgb(255, 165, 0))
            );

            frame.render_widget(
                Table::new(Vec::<Row>::new(), [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(15),
                ])
                    .header(footer)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .block(
                        Block::new()
                            .title("MENU")
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Plain)
                            .padding(Padding::new(1, 1, 1, 1))
                    ),
                task_top[3]
            );
            if app.show_popup {
                pop_up(frame);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            // println!("DEBUG: {:?}", key);
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        if app.show_popup {
                            app.show_popup = false; // close popup
                        } else {
                            break Ok(()); // quit if no popup open
                        }
                    }
                    KeyCode::Char('T') => {
                        return run(terminal, app);
                    }

                    KeyCode::Char('R') => {
                        let task_name = app.textarea.lines().join(" ");
                        if !task_name.trim().is_empty() {
                            if let Some(task) = app.tasks.iter_mut().find(|t| t.name == task_name) {
                                match task.status {
                                    TaskStatus::Paused => {
                                        // Active the task instead of pushing a new one
                                        task.status = TaskStatus::Active;
                                        task.started_at = Some(Instant::now());
                                        app.show_green_log = true;
                                        app.green_log_task = Some(task.name.clone());
                                    }
                                    TaskStatus::Active => {
                                        app.show_popup = true;
                                    }
                                }
                            } else {
                                app.tasks.push(Task {
                                    id: app.next_id,
                                    name: task_name.clone(),
                                    status: TaskStatus::Active,
                                    time_spent: Duration::new(0, 0),
                                    started_at: Some(Instant::now()),
                                    expected_duration: Duration::from_secs(7200),
                                });
                                app.next_id += 1;
                                app.textarea = TextArea::default();
                                app.selected_index = Some(app.tasks.len() - 1);
                                app.focus_textarea = false;
                            }
                        }
                    }

                    KeyCode::Delete => {
                        if let Some(i) = app.selected_index {
                            if i < app.tasks.len() {
                                app.tasks.remove(i);

                                if app.tasks.is_empty() {
                                    app.selected_index = None;
                                } else if i >= app.tasks.len() {
                                    app.selected_index = Some(app.tasks.len() - 1);
                                } else {
                                    app.selected_index = Some(i);
                                }
                            }
                        }
                    }
                    KeyCode::Tab => {
                        app.focus_textarea = !app.focus_textarea; // toggle focus
                    }
                    KeyCode::Down => {
                        if app.focus_textarea {
                            app.textarea.input(key);
                        } else {
                            if let Some(i) = app.selected_index {
                                if i + 1 < app.tasks.len() {
                                    app.selected_index = Some(i + 1);
                                }
                            } else if !app.tasks.is_empty() {
                                app.selected_index = Some(0);
                            }
                        }
                    }
                    KeyCode::Up => {
                        if app.focus_textarea {
                            app.textarea.input(key);
                        } else if let Some(i) = app.selected_index {
                            if i > 0 {
                                app.selected_index = Some(i - 1);
                            }
                        }
                    }

                    _ => {
                        if app.focus_textarea {
                            app.textarea.input(key);
                        }
                    }
                }
            }
        }
    }
}
