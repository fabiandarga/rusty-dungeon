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

use crate::views::components::MessageBlock;

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
        } else {
            self.render_empty_message(frame, rect)?;
        }

        Ok(())
    }

    fn render_empty_message(&mut self, frame: &mut Frame<impl Backend>, rect: Rect) -> Result<(), String> {
        let message_block = MessageBlock::new(
            "Items",
            Span::raw("No items, yet! Go to the dungeon to find some."));
       
        frame.render_widget(message_block, rect);

        Ok(())
    }

    fn render_item_list(&mut self, frame: &mut Frame<impl Backend>, rect: Rect, game_state: &GameState) -> Result<(), String> {
        let layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
        )
        .split(rect);

        let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
        )
        .split(layout_chunks[1]);

        self.list_length = game_state.owned_items.len();


        let list = self.build_item_list(&game_state.owned_items);
        frame.render_stateful_widget(list, layout_chunks[0], &mut self.list_state.clone());

        let details = self.build_item_detail(&game_state.owned_items);
        frame.render_widget(details, right_chunks[0]);


        if game_state.equipped_items.len() > 0 {
            let equipped = self.build_equipped_items(&game_state.equipped_items);
            frame.render_widget(equipped, right_chunks[1]);    
        } else {
            let message_block = MessageBlock::new(
                "Equipment",
                Span::raw("No items equipped!"));
            frame.render_widget(message_block, right_chunks[1]);    
        }
        
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
        let list_state = &self.list_state;

        let selected_item = item_list
        .get(
            list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists") // todo .. two methods. render info if there is no item instead of panicking
        .clone();

    
        Table::new(vec![self.build_item_row(&selected_item)])
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

    fn build_equipped_items(&self, item_list: &Vec<Rc<Item>>) -> Table {
        let rows: Vec<Row> = item_list.iter().map(|item| self.build_item_row(item)).collect();

        Table::new(rows)
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
                .title("Equipment")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
    }

    fn build_item_row(&self, item: &Item) -> Row {
        Row::new(vec![
            Cell::from(Span::raw(item.name.to_string())),
            Cell::from(Span::raw(item.item_type.to_string())),
        ])
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
            KeyCode::Char('e') => {
                if let Some(selected) = list_state.selected() {
                    game_handler.equip_item_by_index(selected)?;
                }
            }
            _ => {}
        }
        
        Ok(true)
    }
}