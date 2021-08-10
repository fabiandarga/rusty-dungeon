use crate::levels::models::{ Level, Room, Item };
use std::rc::Rc;

pub struct GameData {
    levels: Vec<Rc<Level>>,
    rooms: Vec<Rc<Room>>,
    items: Vec<Rc<Item>>,
}

impl GameData {
    pub fn new(levels: Vec<Level>, rooms: Vec<Room>, items:Vec<Item>) -> GameData {
        GameData {
            levels: levels.iter().map(|level| Rc::new(level.clone())).collect(),
            rooms: rooms.iter().map(|room| Rc::new(room.clone())).collect(),
            items: items.iter().map(|item| Rc::new(item.clone())).collect(),
        }
    }

    pub fn find_item_by_id(&self, item_id: u16) -> Option<&Rc<Item>> {
        self.items.iter().find(|item| item.id == item_id)
    }

    pub fn find_level_by_id(&self, level_id: u16) -> Option<&Rc<Level>> {
        self.levels.iter().find(|level| level.id == level_id)
    }


    pub fn find_room_by_id(&self, level_id: u16) -> Option<&Rc<Room>> {
        self.rooms.iter().find(|room| room.id == level_id)
    }
}