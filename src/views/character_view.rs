use crate::GameHandler;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Paragraph, Wrap, Block, Borders, BorderType};
use tui::style::{Style, Modifier, Color};
use tui::layout::{Rect, Alignment};
use tui::backend::Backend;
use tui::Frame;
use crossterm::event::KeyCode;

use crate::Error;
use crate::GameState;

pub struct CharacterView {}

impl CharacterView {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let xp = game_state.xp;

        let content = vec![
            Spans::from(vec![
                Span::styled("Character", Style::default().add_modifier(Modifier::BOLD))
            ]),
            Spans::from(vec![
                Span::raw("Experience Points: "),
                Span::styled(format!("{}", xp), Style::default().fg(Color::Green)),
            ]),
        ];
        
        let p = Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(self.build_block());

        frame.render_widget(p, rect);

        Ok(())
    }

    fn build_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Double)
    }


    pub fn handle_input(
        &self, _key_code: KeyCode, _game_handler: &mut GameHandler) -> Result<bool, Error> {
        Ok(true)
    }
}