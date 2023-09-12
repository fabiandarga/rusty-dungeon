use crate::data::WithId;
use serde::Deserialize;


#[derive(Deserialize, Clone)]
pub struct Enemy {
    pub id: u16,
    pub name: String,
    pub template: String,
    #[serde(skip)]
    pub hp: u16,
    #[serde(alias = "hp")]
    pub hp_max: u16,
    pub strg: u16,
    pub agil: u16,
    pub def: u16,
}

impl Enemy {
    pub fn new(id: u16, name: &str) -> Enemy {
        Enemy { 
            id,
            name: name.to_owned(),
            template: "basic_melee_fighter".to_owned(),
            hp: 0,
            hp_max: 0,
            strg: 0,
            agil: 0,
            def: 0,
        }
    }
}

impl WithId for Enemy {
    fn get_id(&self) -> u16 { self.id }
}
