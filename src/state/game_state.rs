use crate::Error;
use crate::models::models::Reward;
use crate::state::DungeonState;
use std::rc::Rc;
use crate::models::models::{ Level, Room, Item };


pub struct GameState {
    pub xp: u16,
    pub level_points: u16,
    pub dungeon_state: DungeonState,
    pub current_level: Option<Rc<Level>>,
    pub current_room: Option<Rc<Room>>,
    pub owned_items: Vec<Rc<Item>>,
    pub equipped_items: Vec<Rc<Item>>,
    pub last_rewards: Vec<Reward>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            xp: 0,
            level_points: 0,
            dungeon_state: DungeonState::Room,
            current_level: None,
            current_room: None,
            owned_items: Vec::new(),
            equipped_items: Vec::new(),
            last_rewards: Vec::new(),
        }
    }

    pub fn set_current_level(&mut self, level: &Rc<Level>) {
        self.current_level = Some(Rc::clone(level));        
    }

    pub fn get_current_level(&self) -> Result<Level, Error> {
        match &self.current_level {
            Some(level) => {
                Ok((**level).clone())
            },
            None => Err(Error::GameDataError(format!("Trying to access empty current_level")))
        }
    }

    pub fn set_current_room(&mut self, room: &Rc<Room>) {
        self.current_room = Some(Rc::clone(room));
    }
}

