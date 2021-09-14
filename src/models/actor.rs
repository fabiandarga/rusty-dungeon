use crate::models::attack_methods::AttackFunction;
use crate::models::Enemy;
use crate::models::models::Character;

pub trait Actor {
    fn get_actor_values(&self) -> ActorValues;
    fn selectAttack(&self) -> Box<AttackFunction>;
}

#[derive(Debug, Clone)]
pub struct ActorValues {
    pub name: String,
    pub hp: u16,
    pub hp_max: u16,
    pub strg: u16,
    pub agil: u16,
    pub def: u16,

    pub is_player: bool,

    pub initiative: Option<u16>,
}

impl ActorValues {
    pub fn apply_damage(&mut self, dmg: u16) {
        if self.hp < dmg {
            self.hp = 0;
        } else {
            self.hp -= dmg;
        }
    }
}

impl From<&Character> for ActorValues {
    fn from(item: &Character) -> Self {
        ActorValues {
            name: item.name.to_owned(),
            hp: item.hp,
            hp_max: item.hp_max,
            strg: item.strg,
            agil: item.agil,
            def: item.def,
            is_player: true,
            initiative: None,
        }
    }
}

impl From<&Enemy> for ActorValues {
    fn from(item: &Enemy) -> Self {
        ActorValues {
            name: item.name.to_owned(),
            hp: item.hp,
            hp_max: item.hp_max,
            strg: item.strg,
            agil: item.agil,
            def: item.def,
            is_player: false,
            initiative: None,
        }
    }
}