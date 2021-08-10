use crate::Error;
use std::sync::Mutex;
use std::sync::Arc;

use crate::data::GameData;
use crate::state::GameState;
use crate::levels::models::RoomResult::{GainLevelPoints, GainXp, GainItem, GainSkill, StartFight};

pub struct GameHandler {
    game_data: GameData,
    game_state: Arc<Mutex<GameState>>,
}

impl GameHandler {
    pub fn new(game_data: GameData, game_state: Arc<Mutex<GameState>>) -> GameHandler {
        GameHandler { game_data: game_data, game_state: game_state }
    }

    pub fn start_game(&mut self) -> Result<(), String> {
        let level = self.game_data.find_level_by_id(1);
        match level {
            Some(level) => {
                let mut state = self.game_state.lock().unwrap();

                state.set_current_level(level);

                let room_id = level.first_room;
                match self.game_data.find_room_by_id(room_id) {
                    Some(room) => {
                        state.set_current_room(room);
                    }
                    None => return Err(format!("Room {} not found in Data", room_id)),
                }

            },
            None => {
                return Err(String::from("Level 1 not found in Data"))
            }
        };

        return Ok(());
    }

    pub fn execute_room_choice(&mut self, index: usize) -> Result<(), Error> {

        let choices = match &self.game_state.lock().unwrap().current_room {
            Some(room) => room.choices.clone(),
            None => return Err(Error::GameDataError()),
        };

        if choices.len() > index {
            let cons = &choices[index].consequences;
            for c in cons {
                match c {
                    GainLevelPoints(points) => {
                        self.increase_level_points(points);
                    },
                    GainXp(xp) => {
                        self.increase_xp(xp);
                    },
                    GainItem(_id) => {},
                    GainSkill(_id) => {},
                    StartFight(_id) => {},
                }
            }  
        }

        Ok(())
    }

    pub fn increase_level_points(&mut self, points: &u16) {
        self.game_state.lock().unwrap().level_points += points;
    }

    pub fn increase_xp(&mut self, points: &u16) {
        self.game_state.lock().unwrap().xp += points;
    }
}