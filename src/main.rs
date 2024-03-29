use crate::models::Enemy;
use crate::models::Encounter;
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
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{ Block, Borders, ListState, Tabs },
    Terminal,
};

mod models;
use crate::models::models::{ Level, Room, Item, Skill };

mod data;
use crate::data::GameData;

mod game_handler;
use crate::game_handler::GameHandler;

mod global_handler;
use crate::global_handler::GlobalHandler;

mod battle_handler;


mod state;
use crate::state::GameState;

mod views;
use crate::views::{ DungeonView, MenuView, CharacterView, ItemsView };

mod errors;
use crate::errors::Error;

mod random;
mod entities;

const ITEMS_PATH: &str = "./data/items.json";
const ROOMS_PATH: &str = "./data/rooms.json";
const LEVEL_PATH: &str = "./data/levels.json";
const SKILL_PATH: &str = "./data/skills.json";
const ENCOUNTERS_PATH: &str = "./data/encounters.json";
const ENEMIES_PATH: &str = "./data/enemies.json";

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Dungeon,
    Character,
    Items,
    Menu,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Dungeon => 0,
            MenuItem::Character => 1,
            MenuItem::Items => 2,
            MenuItem::Menu => 3,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let levels: Vec<Level> = read_level_db().expect("can fetch level data");
    let rooms: Vec<Room> = read_room_db().expect("can fetch rooms data");
    let items: Vec<Item> = read_item_db().expect("can fetch items data");
    let skills: Vec<Skill> = read_skill_db().expect("can fetch skills data");
    let encounters: Vec<Encounter> = read_encounter_db().expect("can fetch encounter data");
    let enemies: Vec<Enemy> = read_enemies_db().expect("can fetch enemies data");

    let game_data = GameData::new(levels, rooms, items, skills, encounters, enemies);

    let game_handler = Arc::new(Mutex::new(GameHandler::new(game_data)));


    let mut main_game_handler = game_handler.lock().unwrap();
    main_game_handler.start_game().expect("Can start game");
    let state = main_game_handler.get_game_state_clone();
    drop(main_game_handler);

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
    let terminal = Arc::new(Mutex::new(Terminal::new(backend)?));

    let global_terminal = terminal.clone();
    global_terminal.lock().unwrap().clear()?;

    let mut quit = || -> Result<(), Error> {
        disable_raw_mode()?;
        global_terminal.lock().unwrap().show_cursor()?;
        Ok(())
    };

    let mut global_handler = GlobalHandler {
        quit_fn: &mut quit,
    };

    let menu_titles = vec!["Dungeon", "Character", "Items", "Menu"];
    let mut active_menu_item = MenuItem::Dungeon;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));

    let loop_terminal = terminal.clone();
    let loop_game_handler = game_handler.clone();

    let mut items_view = ItemsView::new();

    let mut dungeon_view = DungeonView::new();
    let menu_view = MenuView {};
    let character_view = CharacterView {};

    loop {

        let state = state.clone();

        loop_terminal.lock().unwrap().draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(3)
                .vertical_margin(1)
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
                MenuItem::Dungeon => {
                    dungeon_view.render(frame, chunks[1], &state.lock().unwrap()).expect("To render Dungeon");
                },
                MenuItem::Character => {
                    character_view.render(frame, chunks[1], &state.lock().unwrap()).expect("To render Character");
                },
                MenuItem::Items => {
                    items_view.render(frame, chunks[1], &state.lock().unwrap()).expect("To render Items");
                },
                MenuItem::Menu => {
                    menu_view.render(frame, chunks[1], &state.lock().unwrap()).expect("To render Menu");
                },
            }
        })?;


        match rx.recv()? {
            Event::Input(event) => {
                match event.code {
                    KeyCode::Char('d') => active_menu_item = MenuItem::Dungeon,
                    KeyCode::Char('i') => active_menu_item = MenuItem::Items,
                    KeyCode::Char('m') => active_menu_item = MenuItem::Menu,
                    KeyCode::Char('c') => {
                        if event.modifiers.contains(KeyModifiers::CONTROL) {
                            break; // Break the ui loop
                        } else {
                            active_menu_item = MenuItem::Character
                        }
                     }
                    _ => {}
                };

                match active_menu_item {
                    MenuItem::Dungeon => {
                        dungeon_view.handle_input(event.code, &mut loop_game_handler.lock().unwrap())?;
                    },
                    MenuItem:: Character => {
                        character_view.handle_input(event.code, &mut loop_game_handler.lock().unwrap())?;
                    },
                    MenuItem::Menu => {
                        let res = menu_view.handle_input(event.code, &mut loop_game_handler.lock().unwrap(), &mut global_handler);
                        if let Ok(should_continue) = res {
                            if !should_continue {
                                break;
                            }
                        }
                    },
                    MenuItem::Items => {
                        items_view.handle_input(event.code, &mut loop_game_handler.lock().unwrap())?;
                    },
                };
            },
            Event::Tick => {}
        }
    }


    global_terminal.lock().unwrap().clear()?;

    Ok(())
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

fn read_skill_db() -> Result<Vec<Skill>, Error> {
    let db_content = fs::read_to_string(SKILL_PATH)?;
    let parsed: Vec<Skill> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_encounter_db() -> Result<Vec<Encounter>, Error> {
    let db_content = fs::read_to_string(ENCOUNTERS_PATH)?;
    let parsed: Vec<Encounter> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn read_enemies_db() -> Result<Vec<Enemy>, Error> {
    let db_content = fs::read_to_string(ENEMIES_PATH)?;
    let parsed: Vec<Enemy> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}
