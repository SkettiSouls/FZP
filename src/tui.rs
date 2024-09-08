use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Frame,
    init,
    layout::{Alignment, Rect},
    restore,
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    }
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self) -> io::Result<()> {
        while !self.exit {
            init().draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            // TODO:
            // self.index()?; - Fuzzy Finder
            // self.player()?; - Audio and Visuals (time, now playing, maybe cover art?)
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('Q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_title = Title::from(" Player ".bold());
        let player_block = Block::bordered()
            .title(player_title.alignment(Alignment::Center));
        let placeholder = Text::styled("Under Construction", Style::default().white().add_modifier(Modifier::BOLD));

        Paragraph::new(placeholder)
            .centered()
            .block(player_block)
            .render(area, buf);
    }
}

pub fn start_tui() -> io::Result<()> {
    let app_result = App::default().run();
    restore();
    app_result
}
