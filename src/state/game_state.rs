use crate::Error;
use crate::models::models::Reward;
use crate::state::DungeonState;
use std::rc::Rc;
use crate::models::models::{ Level, Room, Item, Skill, Character, BadResult };
use crate::models::attack_options::*;


pub struct GameState {
    pub level_points: u16,
    pub dungeon_state: DungeonState,
    pub current_level: Option<Rc<Level>>,
    pub current_room: Option<Rc<Room>>,
    pub owned_items: Vec<Rc<Item>>,
    pub equipped_items: Vec<Rc<Item>>,
    pub gained_skills: Vec<Rc<Skill>>,
    pub character: Character,
    pub last_rewards: Vec<Reward>,
    pub last_bad_results: Vec<BadResult>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut character = Character::default();
        character.strg = 2;
        character.agil = 2;
        character.def = 2;

        GameState {
            level_points: 0,
            dungeon_state: DungeonState::Room,
            current_level: None,
            current_room: None,
            owned_items: Vec::new(),
            equipped_items: Vec::new(),
            gained_skills: Vec::new(),
            character,
            last_rewards: Vec::new(),
            last_bad_results: Vec::new(),
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

    pub fn remove_hp(&mut self, hp: u16) {
        if self.character.hp < hp {
            self.character.hp = 0;
        } else {
            self.character.hp -= hp
        }
    }

    pub fn get_attack_options(&self) -> AttackOptions {
        let mut options: AttackOptions = Vec::new();

        options.push(AttackOption::Attack(AttackDescription {
            title: "Unarmed".to_owned(),
            attack_type: "physical".to_owned(),
            dmg_min: 1,
            dmg_max: 3,
            special_effect: "".to_owned(),
        }));

        options.push(AttackOption::None);
        options.push(AttackOption::None);
        options.push(AttackOption::None);

        options
    }
}

