use tui::widgets::Paragraph;
use crossterm::event::KeyCode;
use std::rc::Rc;

use tui::{
    style::{ Style, Color, Modifier },
    widgets::{ Block, BorderType, Borders, Table, Row, Cell, List, ListState, ListItem, },
    text::{ Span, Spans },
    layout::{Layout, Direction, Constraint, Rect, Alignment },
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
        if game_state.owned_items.len() > 0 {
            self.render_item_list(frame, rect, game_state)?;
        }
        self.render_empty_message(frame, rect)?;

        Ok(())
    }

    fn render_empty_message(&mut self, frame: &mut Frame<impl Backend>, rect: Rect) -> Result<(), String> {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Items")
            .border_type(BorderType::Plain);

        let message = Paragraph::new(Span::raw("No items, yet! Go to the dungeon to find some."))
        .alignment(Alignment::Center);

        frame.render_widget(block, rect);

        let chunks = Layout::default()
        .constraints([Constraint::Min(1)])
        .margin(2)
        .split(rect);

        frame.render_widget(message, chunks[0]);

        Ok(())
    }

    fn render_item_list(&mut self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let pets_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
        )
        .split(rect);

        self.list_length = game_state.owned_items.len();


        let list = self.build_item_list(&game_state.owned_items);
        frame.render_stateful_widget(list, pets_chunks[0], &mut self.list_state.clone());

        let details = self.build_item_detail(&game_state.owned_items);
        frame.render_widget(details, pets_chunks[1]);

        Ok(())
    }

    fn build_item_list(&self, item_list: &Vec<Rc<Item>>) -> List {
        let pets = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Items")
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
    
        List::new(items).block(pets).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
    }

    fn build_item_detail(&self,  item_list: &Vec<Rc<Item>>) -> Table {
        let pet_list_state = &self.list_state;

        let selected_pet = item_list
        .get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists") // todo .. two methods. render info if there is no item instead of panicking
        .clone();

    
        Table::new(vec![Row::new(vec![
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
        ])
    }

    pub fn handle_input(&mut self, key_code: KeyCode, _game_handler: &mut GameHandler) -> Result<bool, Error> {
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