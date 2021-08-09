use std::sync::Mutex;
use std::sync::Arc;
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::boxed::Box;
use thiserror::Error;

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

const ITEMS_PATH: &str = "./data/items.json";
const ROOMS_PATH: &str = "./data/rooms.json";
const LEVEL_PATH: &str = "./data/levels.json";

mod levels;
use crate::levels::models::{ Level, Room, Item };

mod data;
use crate::data::GameData;

mod game_handler;
use crate::game_handler::GameHandler;

mod state;
use crate::state::GameState;

mod views;

use crate::views::DungeonView;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
    #[error("error invalid game data")]
    GameDataError(),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Dungeon,
    Items,
    Menu,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Dungeon => 0,
            MenuItem::Items => 1,
            MenuItem::Menu => 2,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let levels: Vec<Level> = read_level_db().expect("can fetch level data");
    let rooms: Vec<Room> = read_room_db().expect("can fetch rooms data");
    let items: Vec<Item> = read_item_db().expect("can fetch items data");

    let game_data = GameData::new(levels, rooms, items);
    let state = Arc::new(Mutex::new(GameState::new()));

    let mut game_handler: GameHandler = GameHandler::new(game_data, state.clone());
    game_handler.start_game().expect("Can start game");

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Dungeon", "Items", "Menu"];
    let mut active_menu_item = MenuItem::Dungeon;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));


    // todo mutex for game state
    loop {

        let dungeon_view = DungeonView {};
        let state = state.clone();

        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(size);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            frame.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Dungeon => dungeon_view.render(frame, chunks[1], &state.lock().unwrap()).expect("To render"),
                MenuItem::Items => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_pets(&pet_list_state);
                    frame.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                    frame.render_widget(right, pets_chunks[1]);
                },
                MenuItem::Menu => frame.render_widget(render_home(), chunks[1])
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('d') => active_menu_item = MenuItem::Dungeon,
                    KeyCode::Char('i') => active_menu_item = MenuItem::Items,
                    KeyCode::Char('m') => active_menu_item = MenuItem::Menu,
                    _ => {}
                };

                match active_menu_item {
                    MenuItem::Dungeon => {
                        dungeon_view.handle_input(event.code);
                    }
                    _ => {}
                };
            },
            Event::Tick => {}
        }
    }


    terminal.clear()?;

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_pets<'a>(pet_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pets")
        .border_type(BorderType::Plain);

    let pet_list = read_item_db().expect("can fetch pet list");
    let items: Vec<_> = pet_list
        .iter()
        .map(|pet| {
            ListItem::new(Spans::from(vec![Span::styled(
                pet.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_pet = pet_list
        .get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
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

fn read_item_db() -> Result<Vec<Item>, Error> {
    let db_content = fs::read_to_string(ITEMS_PATH)?;
    let parsed: Vec<Item> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_level_db() -> Result<Vec<Level>, Error> {
    let db_content = fs::read_to_string(LEVEL_PATH)?;
    let parsed: Vec<Level> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_room_db() -> Result<Vec<Room>, Error> {
    let db_content = fs::read_to_string(ROOMS_PATH)?;
    let parsed: Vec<Room> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}