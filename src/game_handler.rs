use crate::models::models::{ Reward, RewardType };
use crate::state::DungeonState;
use crate::Error;
use std::sync::Mutex;
use std::sync::Arc;

use rand::prelude::*;

use crate::data::GameData;
use crate::state::GameState;
use crate::models::models::RoomResult::{GainLevelPoints, GainXp, GainItem, GainSkill, StartFight};

pub struct GameHandler {
    game_data: GameData,
    game_state: Arc<Mutex<GameState>>,
}

impl GameHandler {
    pub fn new(game_data: GameData, game_state: Arc<Mutex<GameState>>) -> GameHandler {
        GameHandler { game_data: game_data, game_state: game_state }
    }

    pub fn start_game(&mut self) -> Result<(), Error> {
        let level = self.game_data.find_level_by_id(1)?;
        let mut state = self.game_state.lock().unwrap();

        state.set_current_level(level);

        let room_id = level.first_room;
        let room = self.game_data.find_room_by_id(room_id)?;
        state.set_current_room(room);

        Ok(())
    }

    pub fn execute_room_choice(&mut self, index: usize) -> Result<(), Error> {

        let choices = match &self.game_state.lock().unwrap().current_room {
            Some(room) => room.choices.clone(),
            None => return Err(Error::GameDataError(format!("Cant execute choices, no current room set."))),
        };

        let mut rewards: Vec<Reward> = Vec::new();

        if choices.len() > index {
            let cons = &choices[index].consequences;
            for c in cons {
                match c {
                    GainLevelPoints(points) => {
                        self.increase_level_points(points);
                    },
                    GainXp(xp) => {
                        self.increase_xp(xp);

                        rewards.push(Reward {
                            reward_type: RewardType::Xp,
                            name: "Experience Points".to_string(),
                            amount: *xp as usize,
                        });
                    },
                    GainItem(id) => {
                        self.gain_item(id)?;

                        let item = self.game_data.find_item_by_id(*id)?;
                        let name = item.name.to_owned();
                        let item_type = item.item_type.clone();

                        rewards.push(Reward {
                            reward_type: RewardType::Item(item_type),
                            name: name,
                            amount: 1,
                        });
                    },
                    GainSkill(_id) => {
                        // gain skill
                        // let name = self.game_data.find_skill_by_id(*id)?.name.to_owned();
                        // rewards.push(format!("{}", name))
                    },
                    StartFight(_id) => {},
                }
            }  
        }

        if rewards.len() > 0 {
            self.game_state.lock().unwrap().dungeon_state = DungeonState::Result;
        }

        self.game_state.lock().unwrap().last_rewards = rewards;

        // check for levelPoints now and go to final room if needed.
        let room_changed = self.enter_random_room()?;
        if !room_changed {
            self.enter_final_room()?;
        }

        Ok(())
    }

    pub fn enter_random_room(&self) -> Result<bool, Error> {
        
        let opt_level = &self.game_state.lock().unwrap().current_level.clone();
        match opt_level {
            Some(level) => {
                let amount = level.rooms.len();
                if amount == 0 {
                    return Ok(false);
                }
                let random = thread_rng().gen_range(0..amount);
                let room_id = level.rooms[random];
                self.set_current_room(room_id)?;
            }
            None => {
                return Err(Error::GameDataError("No current level set.".to_string()));
            }
        }

        Ok(true)
    }

    fn enter_final_room(&self) -> Result<(), Error>  {
        let opt_level = &self.game_state.lock().unwrap().current_level.clone();
        match opt_level {
            Some(level) => {
                let room_id = level.final_room;
                self.set_current_room(room_id)?;
            }
            None => {
                return Err(Error::GameDataError("No current level set.".to_string()));
            }
        }

        Ok(())
    }

    fn set_current_room(&self, room_id: u16) -> Result<(), Error> {
        let room = self.game_data.find_room_by_id(room_id)?;
        self.game_state.lock().unwrap().set_current_room(room);
        Ok(())
    }

    pub fn set_dungeon_state(&self, ds: DungeonState) {
        self.game_state.lock().unwrap().dungeon_state = ds;
    }

    pub fn get_dungeon_state(&self) -> DungeonState {
        self.game_state.lock().unwrap().dungeon_state.clone()
    }

    pub fn gain_item(&mut self, id: &u16) -> Result<(), Error>{
        let item = self.game_data.find_item_by_id(*id)?;
        self.game_state.lock().unwrap().owned_items.push(item.clone());
        Ok(())
    }

    pub fn increase_level_points(&mut self, points: &u16) {
        self.game_state.lock().unwrap().level_points += points;
    }

    pub fn increase_xp(&mut self, points: &u16) {
        self.game_state.lock().unwrap().xp += points;
    }
}