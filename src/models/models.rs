use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Clone, Display)]
#[serde(tag = "t", content = "c")]
pub enum RoomResult {
    GainLevelPoints(u16),
    GainXp(u16),
    GainItem(u16),
    GainSkill(u16),
    StartFight(u16)
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
pub struct Item {
    pub id: u16,
    pub name: String,
    pub item_type: ItemType,
}

#[derive(Serialize, Deserialize, Display, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum SkillModifier {
    AbilityStr(u8),
    AbilityAgil(u8),
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