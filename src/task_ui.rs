use crate::redundancy_warning::pop_up;
use crate::timer::run_timer;
use crate::util::{ App, Break, Task, TaskStatus };

use color_eyre::Result;

use crossterm::event::{ self, Event, KeyCode, KeyEventKind };

use ratatui::prelude::*;
use ratatui::text::Span;
use ratatui::widgets::{
    Bar,
    BarChart,
    BarGroup,
    Block,
    BorderType,
    Borders,
    Cell,
    List,
    ListItem,
    Padding,
    Paragraph,
    Row,
    Table,
};
use ratatui::{ style::{ Color, Modifier, Style }, DefaultTerminal };

use std::time::{ Duration, Instant };
use tui_textarea::TextArea;

pub fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
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
                        Constraint::Percentage(10), // Header (not focusable)
                        Constraint::Percentage(10), // Nav tabs (focusable row 0)
                        Constraint::Percentage(60), // Task table (focusable row 1)
                        Constraint::Percentage(10), // Command bar (focusable row 2)
                        Constraint::Percentage(10) // Footer (focusable row 3)
                    ]
                )
                .split(layout[0]);

            // HEADER (Not focusable)
            let header_cells = vec!["[Welcome to Chronos]", "[Mode: Tasks]"];
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
                    .title("EDIT TASK")
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
                .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(task_top[2]);

            let task_cells = vec!["[ID]", "[Task Name]", "[Status]", "[Time Spent]"]
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

                    // If active, calculate elapsed time dynamically
                    let elapsed = if let Some(started) = task.started_at {
                        task.time_spent + started.elapsed()
                    } else {
                        task.time_spent
                    };

                    let time_str = {
                        let secs = elapsed.as_secs();
                        format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
                    };

                    let mut row = Row::new(
                        vec![
                            Cell::from(task.id.to_string()),
                            Cell::from(task.name.clone()),
                            Cell::from(status_str),
                            Cell::from(time_str)
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

            //Task-status Panel
            let task_status_panel_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(nested_task_data[1]);

            if let Some(i) = app.selected_index {
                let task = &app.tasks[i];
                let elapsed = if let Some(started) = task.started_at {
                    task.time_spent + started.elapsed()
                } else {
                    task.time_spent
                };

                let time_str = {
                    let secs = elapsed.as_secs();
                    format!("{:02}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
                };

                let items = vec![
                    ListItem::new(format!("[TASK DETAILS]
-------------- ")).style(
                        Style::new().fg(Color::Rgb(255, 165, 0))
                    ),
                    ListItem::new(format!("Task: {}", task.name)).style(
                        Style::new().fg(Color::Yellow)
                    ),
                    ListItem::new(format!("Status: {:?}", task.status)).style(
                        Style::new().fg(Color::Rgb(0, 200, 83))
                    ),
                    ListItem::new(format!("Time: {}", time_str)).style(
                        Style::new().fg(Color::Rgb(173, 216, 230))
                    )
                ];
                let p_pause = Paragraph::new(
                    Text::from(
                        Span::raw("[Press <P> to Pause a task]")
                            .style(Style::new().fg(Color::Red))
                            .add_modifier(Modifier::BOLD)
                    )
                );

                let p_resume = Paragraph::new(
                    Text::from(
                        Span::raw("[Press <R> to Resume a task]")
                            .style(Style::new().fg(Color::Rgb(34, 139, 34)))
                            .add_modifier(Modifier::BOLD)
                    )
                );

                //Rendering bar charts
                let mut bars: Vec<Bar> = Vec::new();
                let block = Block::default()
                    .title("ELAPSED TIME BAR GRAPH")
                    .borders(Borders::ALL)
                    .padding(Padding::new(2, 0, 5, 0));

                for task in &app.tasks {
                    let elapsed = if let Some(started) = task.started_at {
                        task.time_spent + started.elapsed()
                    } else {
                        task.time_spent
                    };

                    let minutes = (elapsed.as_secs() / 60) as u64;

                    bars.push(
                        Bar::default()
                            .value(minutes)
                            .label(Line::from(task.name.as_str()))
                            .text_value(format!("{minutes}m"))
                    );
                }
                // Put bars into a group
                let group = BarGroup::default().bars(&bars);

                let chart = BarChart::default()
                    .block(block)
                    .data(group)
                    .bar_width(5) // width of each bar (columns)
                    .bar_gap(3) // gap between bars in the same group
                    .group_gap(3) // gap between groups (useful for multi-series)
                    .max(20) // top of the scale (choose >= max value)
                    .label_style(Style::default().fg(Color::Rgb(102, 217, 239)))
                    .bar_style(Style::default().fg(Color::Yellow))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Yellow).bold());

                frame.render_widget(
                    List::new(items).block(
                        //.style(Style::new().fg(Color::Rgb(0, 200, 180)))
                        Block::new()
                            .title("STATUS")
                            .borders(
                                Borders::TOP | Borders::LEFT | Borders::RIGHT | Borders::BOTTOM
                            )
                            .border_type(ratatui::widgets::BorderType::Thick)
                            .padding(Padding::new(2, 2, 1, 1))
                    ),
                    task_status_panel_layout[0]
                );
                frame.render_widget(
                    p_pause
                        .block(
                            Block::default()
                                .title("STATUS")
                                .borders(Borders::ALL)
                                .padding(Padding::new(0, 2, 5, 0))
                        )
                        .alignment(Alignment::Right),
                    task_status_panel_layout[0]
                );
                frame.render_widget(
                    p_resume
                        .block(
                            Block::default()
                                .title("STATUS")
                                .borders(Borders::ALL)
                                .padding(Padding::new(0, 2, 0, 5))
                        )
                        .alignment(Alignment::Right),
                    task_status_panel_layout[0]
                );
                frame.render_widget(chart, task_status_panel_layout[1]);
            } else {
                // No task selected → show empty state
                let list = List::new(vec![ListItem::new("Task list")]).block(
                    Block::default().title("STATUS").borders(Borders::ALL)
                );

                frame.render_widget(list, task_status_panel_layout[0]);

                let stat_list = List::new(vec![ListItem::new("All task Stats")]).block(
                    Block::default().title("ELAPSED TIME BAR GRAPH").borders(Borders::ALL)
                );

                frame.render_widget(stat_list, task_status_panel_layout[1]);
            }

            // COMMAND BAR (Focusable row 2)
            let command_cells = vec!["Command: "];
            let command_cells_spans: Vec<Span> = command_cells
                .iter()
                .map(|h| Span::raw(*h))
                .collect();

            let command_header = Row::new(command_cells_spans).style(
                Style::new().fg(Color::Rgb(102, 217, 239))
            );

            frame.render_widget(
                Table::new(Vec::<Row>::new(), [Constraint::Percentage(90)])
                    .header(command_header)
                    .style(Style::new().fg(Color::Rgb(0, 200, 180)))
                    .block(
                        Block::new()
                            .title("COMMAND")
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Plain)
                            .padding(Padding::new(1, 1, 1, 1))
                    ),
                task_top[3]
            );

            // FOOTER MENU (Focusable row 3)
            let footer_cells = vec![
                "<Esc> Exit",
                "<UP/DOWN> Move",
                "<Tab> Focus",
                "<Enter> Add Task",
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
                task_top[4]
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
                        return run_timer(terminal, app);
                    }
                    KeyCode::Enter => {
                        let task_name = app.textarea.lines().join(" "); // take input from textarea
                        if !task_name.trim().is_empty() {
                            app.tasks.push(Task {
                                id: app.next_id,
                                name: task_name,
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

                    KeyCode::Char('P') => {
                        let task_name = app.textarea.lines().join(" ");
                        if !task_name.trim().is_empty() {
                            if
                                let Some((i, task)) = app.tasks
                                    .iter_mut()
                                    .enumerate()
                                    .find(|(_, t)| t.name == task_name)
                            {
                                match task.status {
                                    TaskStatus::Active => {
                                        // Pause the task instead of pushing a new one
                                        task.status = TaskStatus::Paused;
                                        task.time_spent += task.started_at.unwrap().elapsed();
                                        task.started_at = None;

                                        // ✅ Start a break timer for THIS task
                                        app.breaks.insert(i as u32, Break::new());
                                    }
                                    TaskStatus::Paused => {
                                        // Already paused → show popup, don't add
                                        app.show_popup = true;
                                    }
                                }
                            } else {
                                // No such task exists → insert as new Paused task
                                app.tasks.push(Task {
                                    id: app.next_id,
                                    name: task_name.clone(),
                                    status: TaskStatus::Paused,
                                    time_spent: Duration::new(0, 0),
                                    started_at: Some(Instant::now()),
                                    expected_duration: Duration::from_secs(7200),
                                });
                                app.next_id += 1;
                                app.textarea = TextArea::default();
                                app.selected_index = Some(app.tasks.len() - 1);
                                app.focus_textarea = false;

                                // ✅ New task paused immediately → add its break state too
                                app.breaks.insert((app.tasks.len() - 1) as u32, Break::new());
                            }
                        }
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
                                    }
                                    TaskStatus::Active => {
                                        // Already paused → show popup, don't add
                                        app.show_popup = true;
                                    }
                                }
                            } else {
                                // No such task exists → insert as new Paused task
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
