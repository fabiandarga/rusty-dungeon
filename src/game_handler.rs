use std::sync::Mutex;
use std::sync::Arc;
use crate::data::GameData;
use crate::state::GameState;

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
}