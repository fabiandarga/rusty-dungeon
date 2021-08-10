use crossterm::event::KeyCode;
use std::rc::Rc;

use tui::{
    style::{ Style, Color, Modifier },
    widgets::{ Block, BorderType, Borders, Table, Row, Cell, List, ListState, ListItem },
    text::{ Span, Spans },
    layout::{Layout, Direction, Constraint, Rect},
    backend::Backend,
    Frame,
};

use crate::Error;
use crate::GameHandler;
use crate::GameState;
use crate::Item;

pub struct ItemsView {
    list_state: ListState,
    list_length: usize,
}

impl ItemsView {
    pub fn new() -> ItemsView {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        ItemsView {
            list_state,
            list_length: 0
        }
    }

    pub fn render(&mut self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let pets_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
            )
            .split(rect);

        self.list_length = game_state.owned_items.len();

        let (left, right) = self.render_pets(&game_state.owned_items);
        frame.render_stateful_widget(left, pets_chunks[0], &mut self.list_state);
        frame.render_widget(right, pets_chunks[1]);

        Ok(())
    }

    fn render_pets<'a>(&self, item_list: &Vec<Rc<Item>>) -> (List<'a>, Table<'a>) {
        let pet_list_state = &self.list_state;

        let pets = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Pets")
            .border_type(BorderType::Plain);
    
        let items: Vec<_> = item_list
            .iter()
            .map(|pet| {
                ListItem::new(Spans::from(vec![Span::styled(
                    pet.name.clone(),
                    Style::default(),
                )]))
            })
            .collect();
    
        let selected_pet = item_list
            .get(
                pet_list_state
                    .selected()
                    .expect("there is always a selected pet"),
            )
            .expect("exists") // todo .. two methods. render info if there is no item instead of panicking
            .clone();
    
        let list = List::new(items).block(pets).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    
        let pet_detail = Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(selected_pet.name.to_string())),
            Cell::from(Span::raw(selected_pet.item_type.to_string())),
        ])])
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Name",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Category",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]);
    
        (list, pet_detail)
    }

    pub fn handle_input(&mut self, key_code: KeyCode, game_handler: &mut GameHandler) -> Result<bool, Error> {
        let list_state = &mut self.list_state;

        match key_code {
            KeyCode::Down => {
                if let Some(selected) = list_state.selected() {
                    let amount_pets = self.list_length;
                    if selected >= amount_pets - 1 {
                        list_state.select(Some(0));
                    } else {
                        list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Up => {
                if let Some(selected) = list_state.selected() {
                    let amount_pets = self.list_length;
                    if selected > 0 {
                        list_state.select(Some(selected - 1));
                    } else {
                        list_state.select(Some(amount_pets - 1));
                    }
                }
            }
            _ => {}
        }
        
        Ok(true)
    }
}