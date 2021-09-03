use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Ability {
    Strg,
    Agil,
    Def,
}

#[derive(Serialize, Deserialize, Clone, Display)]
#[serde(tag = "t", content = "c")]
pub enum RoomResult {
    GainLevelPoints(u16),
    GainXp(u16),
    GainItem(u16),
    GainSkill(u16),
    StartFight(u16),
    AbilityCheck(Ability, u8),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Choice {
    pub text: String,
    pub consequences: Vec<RoomResult>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: u16,
    pub title: String,
    pub text: String,
    pub choices: Vec<Choice>,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub name: String,
    pub id: u16,
    pub rooms: Vec<u16>,
    pub level_points: u16,
    pub first_room: u16,
    pub final_room: u16,
}

#[derive(Serialize, Deserialize, Display, Clone, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: u16,
    pub name: String,
    pub hp: u16,
    pub hp_max: u16,
    pub xp: u16,
    pub strg: u16,
    pub agil: u16,
    pub def: u16,
}

impl Character {
    pub fn default() -> Character {
        Character {
            id: 0,
            name: "Barbie".to_owned(),
            hp: 100,
            hp_max: 100,
            xp: 0,
            strg: 2,
            agil: 2,
            def: 2,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u16,
    pub name: String,
    pub item_type: ItemType,

    #[serde(default = "default_ability")]
    pub strg: u16,

    #[serde(default = "default_ability")]
    pub agil: u16,
    
    #[serde(default = "default_ability")]
    pub def: u16,
}

fn default_ability() -> u16 {
    0
}

#[derive(Serialize, Deserialize, Display, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum SkillModifier {
    AbilityIncrease(Ability, u8),
    AbilityMulti(Ability, u8),
    LPIncrease(u8),
    XPIncrease(u8),
    DropChance(u8),
    CritChance(u8),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    pub id: u16,
    pub name: String,
    pub modifiers: Vec<SkillModifier>,
}

#[derive(Clone)]
pub enum RewardType {
    Item(ItemType),
    Skill,
    Xp,
}

#[derive(Clone)]
pub struct Reward {
    pub reward_type: RewardType,
    pub name: String,
    pub amount: usize,
}

#[derive(Clone)]
pub enum BadResultType {
    Damage,
    Skill,
}

#[derive(Clone)]
pub struct BadResult {
    pub bad_result_type: BadResultType,
    pub name: String,
    pub amount: usize,
}