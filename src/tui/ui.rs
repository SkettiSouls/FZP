use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame) {
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(2),
        ])
        .split(frame.area());

    let index_block = Block::default()
        .borders(Borders::ALL)
        .title(" Search Index" )
        .style(Style::default());

    let placeholder = Paragraph::new(Text::styled(
        "Under Construction",
        Style::default().fg(Color::Green),
    ));

    let now_playing_block = Block::bordered()
        .title(" Now Playing ")
        .style(Style::default());

    frame.render_widget(placeholder.clone().block(index_block), panes[0]);
    frame.render_widget(placeholder.block(now_playing_block), panes[1]);
}
