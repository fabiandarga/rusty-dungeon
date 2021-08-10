use crate::GameHandler;
use crate::Error;
use tui::text::Spans;
use tui::text::Span;
use crate::levels::models::Choice;
use tui::widgets::Wrap;
use crate::GameState;
use crossterm::event::KeyCode;

use tui::{
    Frame,
    layout::{ Rect, Layout, Direction, Constraint, Alignment },
    style::{Color, Style, Modifier},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
};

use ::tui::backend::Backend;

pub struct DungeonView {}

impl DungeonView {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let dungeon_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Length(5), Constraint::Min(2), Constraint::Length(5)].as_ref(),
        )
        .split(rect);

        let room = match &game_state.current_room {
            Some(room) => room,
            None => return Err("No Room found".to_string()),
        };

        frame.render_widget(self.build_title(&room.title), dungeon_chunks[0]);
        frame.render_widget(self.build_body(&room.text), dungeon_chunks[1]);
        frame.render_widget(self.build_choice_widget(&room.choices), dungeon_chunks[2]);

        Ok(())
    }

    fn build_title(&self, title: &str) -> Paragraph {
        Paragraph::new(title.to_owned())
            .style(Style::default().add_modifier(Modifier::BOLD))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(self.build_block())
    }

    fn build_body(&self, text: &str) -> Paragraph {
        Paragraph::new(text.to_owned())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(self.build_block())
    }

    fn build_choice_widget(&self, choices: &[Choice]) -> Paragraph {
        let mut content: Vec<Span> = Vec::new();

        choices.iter().enumerate().for_each(|(idx,choice)| {
            if idx > 0 {
                content.push(Span::raw("  |  "));
            }
            content.push(Span::styled("[", Style::default().fg(Color::Yellow)));
            content.push(Span::styled(
                format!("{}", idx + 1),
                Style::default().add_modifier(Modifier::BOLD)));
            content.push(Span::styled("]", Style::default().fg(Color::Yellow)));
            content.push(Span::styled(format!(" {}", choice.text), Style::default().add_modifier(Modifier::ITALIC)));
        });

        Paragraph::new(Spans::from(content))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            )
    }

    fn build_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Double)
    }

    pub fn handle_input(&mut self, key_code: KeyCode, game_handler: &mut GameHandler) -> Result<bool, Error> {
        match key_code {
            KeyCode::Char('1') => {
                game_handler.execute_room_choice(0)?;
            }
            KeyCode::Char('2') => {
                game_handler.execute_room_choice(1)?;
            }
            KeyCode::Char('3') => {
                game_handler.execute_room_choice(2)?;
            }
            KeyCode::Char('4') => {
                game_handler.execute_room_choice(3)?;
            }
            _ => {}
        }

        Ok(true)
    }
}

