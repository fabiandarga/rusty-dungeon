use crate::GameState;
use crossterm::event::KeyCode;

use tui::{
    Frame,
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    style::{Color, Style},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
};

use ::tui::backend::Backend;

pub struct DungeonView {

}

impl DungeonView {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let dungeon_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Min(2), Constraint::Length(5)].as_ref(),
        )
        .split(rect);

        let room = match &game_state.current_room {
            Some(room) => room,
            None => return Err("No Room found".to_string()),
        };

        let main = Paragraph::new(room.text.to_string())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        let choices = Paragraph::new("Choices")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );
        frame.render_widget(main, dungeon_chunks[0]);
        frame.render_widget(choices, dungeon_chunks[1]);

        Ok(())
    }

    pub fn handle_input(&self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('1') => {
                println!("1");

            }
            KeyCode::Char('2') => {
                println!("2");
            }
            _ => {}
        }
    }
}

