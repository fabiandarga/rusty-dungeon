use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Paragraph, Wrap, Block, Borders, BorderType};
use tui::style::{Style, Modifier, Color};
use tui::layout::{Rect, Alignment};
use tui::backend::Backend;
use tui::Frame;
use crossterm::event::KeyCode;

use crate::Error;
use crate::GlobalHandler;
use crate::GameState;

pub struct MenuView {}

impl MenuView {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, _game_state: &GameState) -> Result<(), String> {
        let content = vec![
            Spans::from(vec![
                Span::styled("Menu", Style::default().add_modifier(Modifier::BOLD))
            ]),
            Spans::from(vec![
                Span::styled("[q]", Style::default().fg(Color::Cyan)),
                Span::raw(" Quit"),
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
        &self, key_code: KeyCode, global_handlers: &mut GlobalHandler)
        -> Result<bool, Error> {

        match key_code {
            KeyCode::Char('q') => {
                global_handlers.quit()?;
                return Ok(false);
            }
            _ => {}
        }

        Ok(true)
    }
}