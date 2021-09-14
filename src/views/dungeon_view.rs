use crate::models::models::{ Reward, RewardType, Choice, ItemType, BadResult, BadResultType };
use crate::GameHandler;
use crate::Error;
use crate::views::dungeon::BattleView;
use tui::text::Spans;
use tui::text::Span;
use tui::widgets::Wrap;
use crate::state::{ GameState, DungeonState };
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

pub struct DungeonView {
    battle_view: BattleView,
}

impl DungeonView {
    pub fn new() -> DungeonView {
        DungeonView {
            battle_view: BattleView::new(),
        }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), Error> {
        match game_state.dungeon_state {
            DungeonState::Room => {
                self.render_room(frame, rect, game_state)?;
            }
            DungeonState::Result => {
                self.render_result_screen(frame, rect, game_state);
            }
            DungeonState::Failure => {
                self.render_failure_screen(frame, rect, game_state);
            }
            DungeonState::Encounter => {
                self.battle_view.render(frame, rect, game_state)?;
            }
        }
        Ok(())
    }

    fn render_room(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), Error> {
        let dungeon_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Length(3), Constraint::Min(2), Constraint::Length(5)].as_ref(),
            )
            .split(rect);

        let room = match &game_state.current_room {
            Some(room) => room,
            None => return Err(Error::GameDataError("No Room found".to_string())),
        };

        frame.render_widget(self.build_title(&room.title), dungeon_chunks[0]);
        frame.render_widget(self.build_body(&room.text), dungeon_chunks[1]);
        frame.render_widget(self.build_choice_widget(&room.choices), dungeon_chunks[2]);

        Ok(())
    }

    fn render_result_screen(&self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) {
        let dungeon_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Min(2), Constraint::Length(5)].as_ref(),
        )
        .split(rect);

        frame.render_widget(self.build_result_widget(&game_state.last_rewards), dungeon_chunks[0]);
        frame.render_widget(self.build_confirm_widget(), dungeon_chunks[1]);
    }

    fn render_failure_screen(&self, frame: &mut Frame<impl Backend>, rect:Rect, game_state: &GameState) {
        let dungeon_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Length(3), Constraint::Min(2), Constraint::Length(5)].as_ref(),
        )
        .split(rect);

        let title = Paragraph::new("You failed!")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            );

        frame.render_widget(title, dungeon_chunks[0]);
        frame.render_widget(self.build_failure_widget(&game_state.last_bad_results), dungeon_chunks[1]);
        frame.render_widget(self.build_confirm_widget(), dungeon_chunks[2]);
    }

    fn build_result_widget(&self, last_rewards: &Vec<Reward>) -> Paragraph {
        let mut content: Vec<Spans> = Vec::new();

        for reward in last_rewards {
            let mut reward_text: Vec<Span> = Vec::new();
            if reward.amount > 1 {
                reward_text.push(Span::raw(format!("{} ", reward.amount)))
            }
            let color = match &reward.reward_type {
                RewardType::Xp => Color::LightGreen,
                RewardType::Item(it) => {
                    match it {
                        ItemType::Armor => Color::LightYellow,
                        ItemType::Weapon => Color::LightRed,
                    }
                }
                _ => Color::White,
            };
            reward_text.push(Span::styled(format!("{}", reward.name), Style::default().fg(color)));

            content.push(Spans::from(reward_text));
        }
        
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            )
    }

    fn build_failure_widget(&self, last_bad_results: &Vec<BadResult>) -> Paragraph {
        let mut content: Vec<Spans> = Vec::new();

        for result in last_bad_results {
            let mut text: Vec<Span> = Vec::new();
            if result.amount > 1 {
                text.push(Span::raw(format!("{} ", result.amount)))
            }
            let color = match &result.bad_result_type {
                BadResultType::Damage => Color::LightRed,
                _ => Color::White,
            };
            text.push(Span::styled(format!("{}", result.name), Style::default().fg(color)));

            content.push(Spans::from(text));
        }

        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            )

    }

    fn build_confirm_widget(&self) -> Paragraph {
        let content: Vec<Span> = vec![
            Span::styled("[", Style::default().fg(Color::Yellow)),
            Span::styled(format!("1"), Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("]", Style::default().fg(Color::Yellow)),
            Span::raw(" OK")
        ];

        Paragraph::new(Spans::from(content))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded),
            )
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
        let state = game_handler.get_dungeon_state();
        match state {
            DungeonState::Room => {
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
            }
            DungeonState::Encounter => {
                self.battle_view.handle_input(key_code, game_handler)?;
            }
            DungeonState::Result | DungeonState::Failure => {
                match key_code {
                    KeyCode::Char('1') => {
                        game_handler.set_dungeon_state(DungeonState::Room);
                    }
                    _ => {}
                }
            }
        }

        Ok(true)
    }
}

