use crate::task_ui::run;
use crate::util::{ App };
use crossterm::event::{ self, Event, KeyCode, KeyEventKind };
use ratatui::layout::Alignment;
use ratatui::prelude::*;
use ratatui::text::{ Line, Span, Text };
use ratatui::widgets::{ Block, Borders, Padding, Paragraph };
use ratatui::{ style::{ Color, Modifier, Style }, DefaultTerminal };

use std::time::Duration;

pub fn tui_banner(mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let banner_ascii =
        r#"
 ██████╗██╗  ██╗██████╗  ██████╗ ███╗   ██╗ ██████╗ ███████╗
██╔════╝██║  ██║██╔══██╗██╔═══██╗████╗  ██║██╔═══██╗██╔════╝
██║     ███████║██████╔╝██║   ██║██╔██╗ ██║██║   ██║███████╗
██║     ██╔══██║██╔══██╗██║   ██║██║╚██╗██║██║   ██║╚════██║
╚██████╗██║  ██║██║  ██║╚██████╔╝██║ ╚████║╚██████╔╝███████║
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
"#;
    let task_ui_app_instance = App::new();

    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(100)])
                .split(frame.area());

            // Background
            frame.render_widget(
                Block::default().style(Style::default().bg(Color::Rgb(10, 14, 32))),
                layout[0]
            );

            let nest = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    vec![
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ]
                )
                .split(layout[0]);

            // Banner area
            let banner_block = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Length(25), Constraint::Fill(1)])
                .split(nest[1]);

            // Split ASCII into separate lines
            let mut banner_lines: Vec<Line> = banner_ascii
                .lines()
                .map(|l| {
                    Line::from(Span::styled(l, Style::default().fg(Color::Rgb(255, 165, 0))))
                })
                .collect();

            // Add spacing after ASCII
            banner_lines.push(Line::from(""));

            // Add welcome text lines
            banner_lines.extend(
                vec![
                    Line::from(
                        Span::styled(
                            "Welcome to Tessa, your personal time tracker.",
                            Style::default().fg(Color::Rgb(0, 200, 180))
                        )
                    ),
                    Line::from(
                        Span::styled(
                            "Set new tasks and keep records of your productivity at your fingertips.",
                            Style::default().fg(Color::Rgb(0, 200, 180))
                        )
                    ),
                    Line::from(
                        Span::styled(
                            "Let's get started, shall we?",
                            Style::default().fg(Color::Rgb(0, 200, 180))
                        )
                    ),
                    Line::from(
                        Span::styled(
                            "Jump to dashboard by pressing D on your keyboard.",
                            Style::default().fg(Color::Rgb(0, 200, 180))
                        )
                    ),
                    Line::from(""), // extra space
                    Line::from(
                        Span::styled(
                            "💡 Tip: Stay consistent — little progress each day adds up to big results.",
                            Style::default()
                                .fg(Color::Rgb(255, 255, 150))
                                .add_modifier(Modifier::BOLD)
                        )
                    ),
                    Line::from(
                        Span::styled(
                            "▶ Track it.  📈 Improve it.  🚀  Own your time.",
                            Style::default()
                                .fg(Color::Rgb(255, 255, 150))
                                .add_modifier(Modifier::BOLD)
                        )
                    ),
                    Line::from(""),
                    Line::from(""),
                    Line::from(""),
                    Line::from(
                        Span::styled(
                            "Press <D> to jump to DASHBOARD",
                            Style::default().fg(Color::Rgb(255, 165, 0))
                        )
                    )
                ]
            );

            let banner_widget = Paragraph::new(Text::from(banner_lines))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(255, 165, 0)))
                .block(
                    Block::new()
                        .title("BANNER")
                        .borders(Borders::ALL)
                        .padding(Padding::new(1, 1, 1, 1))
                );

            frame.render_widget(banner_widget, banner_block[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('D') => {
                            // Exit tui_banner loop and hand over the terminal to run()
                            return run(terminal, task_ui_app_instance);
                        }
                        KeyCode::Esc => {
                            break Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
