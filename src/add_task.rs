use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
pub fn pop_up(frame: &mut Frame) {
    let area = centered_rect(60, 20, frame.size()); // 60% width, 20% height
    frame.render_widget(Clear, area); // Clears the background
    let block = Block::default()
        .title("WARNING")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Red));
    let paragraph = Paragraph::new("A paused task with the same name already exists!")
        .centered()
        .alignment(Alignment::Center);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, inner); // Render content inside block
}
