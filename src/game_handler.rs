use crate::models::models::{ Reward, RewardType, Ability, BadResult, BadResultType };
use crate::state::DungeonState;
use crate::Error;
use std::sync::Mutex;
use std::sync::Arc;

use rand::prelude::*;

use crate::data::GameData;
use crate::state::GameState;
use crate::models::models::RoomResult::*;

use crate::random::ability_check_with_nd6;

pub struct GameHandler {
    game_data: GameData,
    game_state: Arc<Mutex<GameState>>,
}

impl GameHandler {
    pub fn new(game_data: GameData) -> GameHandler {
        let state = Arc::new(Mutex::new(GameState::new()));
        GameHandler { game_data: game_data, game_state: state }
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

    pub fn reset_game(&mut self) -> Result<(), Error> {
        let mut state = self.game_state.lock().expect("Could not lock game_State");
        *state = GameState::new();
        drop(state);
        self.start_game()?;
        Ok(())
    }

    pub fn execute_room_choice(&mut self, index: usize) -> Result<(), Error> {

        let choices = match &self.game_state.lock().unwrap().current_room {
            Some(room) => room.choices.clone(),
            None => return Err(Error::GameDataError(format!("Cant execute choices, no current room set."))),
        };

        let mut rewards: Vec<Reward> = Vec::new();
        let mut bad_results: Vec<BadResult> = Vec::new();

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
                        let item_type = item.item_type.clone();

                        rewards.push(Reward {
                            reward_type: RewardType::Item(item_type),
                            name: item.name.to_owned(),
                            amount: 1,
                        });
                    },
                    GainSkill(id) => {
                        // gain skill
                        let gained = self.gain_skill_once(id)?;

                        if gained {
                            let skill = self.game_data.find_skill_by_id(*id)?;

                            rewards.push(Reward {
                                reward_type: RewardType::Skill,
                                name: skill.name.to_owned(),
                                amount: 1,
                            });
                        }
                    },
                    StartFight(_id) => {},
                    AbilityCheck(ability, dificulty) => {
                        let success = self.ability_check(ability.clone(), *dificulty)?;
                        if !success {
                            let damage = self.recive_damage(*dificulty);
                            bad_results.push(BadResult {
                                bad_result_type: BadResultType::Damage,
                                name: "Damage".to_string(),
                                amount: damage.into(),
                            });

                            self.change_dungeon_state(DungeonState::Failure);
                            break;
                        }
                    },
                }
            }  
        }

        if rewards.len() > 0 {
            self.change_dungeon_state(DungeonState::Result);
        }

        let mut gs = self.game_state.lock().unwrap();

        gs.last_bad_results = bad_results;
        gs.last_rewards = rewards;
        drop(gs);

        self.change_room()?;

        Ok(())
    }

    fn ability_check(&self, ability: Ability, dificulty: u8) -> Result<bool, Error> {
        let character = self.game_state.lock().unwrap().character.clone();
        let success = ability_check_with_nd6(character, ability, dificulty);
        Ok(success)
    }

    fn change_room(&self) -> Result<(), Error>  {
        // check for levelPoints now and go to final room if needed.
        let gs = self.game_state.lock().unwrap();
        let level_points = gs.level_points;
        let level = gs.get_current_level()?;

        drop(gs);

        if level.rooms.len() == 0 || level_points >= level.level_points {
            self.enter_final_room()?;
        } else {
            self.enter_random_room()?;
        }
        
        Ok(())
    }

    pub fn enter_random_room(&self) -> Result<(), Error> {
        let level = &self.game_state.lock().unwrap().get_current_level()?;
        let amount = level.rooms.len();
        if amount == 0 {
            return Err(Error::GameDataError("Trying to enter random room, but no rooms exist.".to_string()));
        }
        let random = thread_rng().gen_range(0..amount);
        let room_id = level.rooms[random];
        self.set_current_room(room_id)?;

        Ok(())
    }

    pub fn change_dungeon_state(&self, state: DungeonState) {
        self.game_state.lock().unwrap().dungeon_state = state;
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

    pub fn get_game_state_clone(&self) -> Arc<Mutex<GameState>> {
        self.game_state.clone()
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

    pub fn recive_damage(&self, dificulty: u8) -> u16 {
        // TODO extract hardcoded ranges
        let range = match dificulty {
            1 | 2 => 0..=10,
            3 => 8..=20,
            4 => 15..=30,
            _ => 20..=50,
        };

        let random_dmg = thread_rng().gen_range(range);

        self.game_state.lock().unwrap().remove_hp(random_dmg);

        random_dmg
    }

    pub fn gain_item(&mut self, id: &u16) -> Result<(), Error> {
        let item = self.game_data.find_item_by_id(*id)?;
        self.game_state.lock().unwrap().owned_items.push(item.clone());
        Ok(())
    }

    pub fn gain_skill_once(&mut self, id: &u16) -> Result<bool, Error> {
        let skill = self.game_data.find_skill_by_id(*id)?;
        let mut gs = self.game_state.lock().unwrap();
        if !gs.gained_skills.iter().any(|skill| &skill.id == id) {
            gs.gained_skills.push(skill.clone());
            return Ok(true);
        }

        Ok(false)
    }

    pub fn increase_level_points(&mut self, points: &u16) {
        self.game_state.lock().unwrap().level_points += points;
    }

    pub fn increase_xp(&mut self, points: &u16) {
        self.game_state.lock().unwrap().character.xp += points;
    }

    pub fn equip_item_by_index(&self, index: usize) -> Result<(), Error> {
        let id = match self.game_state.lock().unwrap().owned_items.get(index) {
            Some(item) => item.id,
            _ => return Err(Error::GameDataError(format!("Can not equip item with index {}", index))),
        };

        self.equip_item(id)?;

        Ok(())
    }

    pub fn equip_item(&self, item_id: u16) -> Result<(), Error>{
        if self.has_item(item_id) {
            // check type. To replace equipped item of that type. 
            let item = self.game_data.find_item_by_id(item_id)?;
            let mut gs = self.game_state.lock().unwrap();
            let old_item_index = gs.equipped_items.iter()
                .position(|old_item| old_item.item_type == item.item_type);

            match old_item_index {
                Some(index) => {
                    gs.equipped_items.remove(index);
                }
                None => {}
            }

            gs.equipped_items.push(item.clone());
            return Ok(());
        }
        Err(Error::GameDataError("Trying to equip item not owned".to_string()))
    }

    pub fn has_item(&self, item_id: u16) -> bool {
        let first_index = self.game_state.lock().unwrap()
            .owned_items.iter()
            .position(|item| item.id == item_id);
        match first_index {
            Some(_index) => true,
            None => false,
        }
    }
}
