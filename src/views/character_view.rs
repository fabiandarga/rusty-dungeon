use std::rc::Rc;
use crate::Skill;
use crate::GameHandler;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Paragraph, Wrap, Block, Borders, BorderType};
use tui::style::{Style, Modifier, Color};
use tui::layout::{Layout, Rect, Alignment, Direction, Constraint};
use tui::backend::Backend;
use tui::Frame;
use crossterm::event::KeyCode;

use crate::Error;
use crate::GameState;

pub struct CharacterView {}

impl CharacterView {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let xp = game_state.xp;
        let overview = self.build_overview(xp);

        let skills = self.build_skill_section(&game_state.gained_skills[..]);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(25), Constraint::Min(25)])
            .split(rect);

        frame.render_widget(overview, chunks[0]);
        frame.render_widget(skills, chunks[1]);

        Ok(())
    }

    fn build_overview(&self, xp: u16) -> Paragraph {
        let content = vec![
            Spans::from(vec![
                Span::styled("Character", Style::default().add_modifier(Modifier::BOLD))
            ]),
            Spans::from(vec![
                Span::raw("Experience Points: "),
                Span::styled(format!("{}", xp), Style::default().fg(Color::Green)),
            ]),
        ];
        
        Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(self.build_block())
    }

    fn build_skill_section(&self, skill_list: &[Rc<Skill>]) -> Paragraph {

        let mut skills = Vec::new();
        skill_list.iter().for_each(|skill| skills.push(Span::raw(skill.name.to_owned())));

        let content = vec![
            Spans::from(vec![
                Span::styled("Features", Style::default().add_modifier(Modifier::BOLD))
            ]),
            Spans::from(skills)
        ];

        Paragraph::new(content)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(self.build_block())
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