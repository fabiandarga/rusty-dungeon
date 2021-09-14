#[derive(Debug, Clone)]
pub struct BattleEvents {
    pub events: Vec<BattleEvent>
}

impl BattleEvents {
    pub fn new() -> BattleEvents {
        BattleEvents { events: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct BattleEvent {
    pub title: String,
    pub source: String,
    pub target: String,
    pub effect: String,
}

impl BattleEvent {
    pub fn default() -> BattleEvent {
        BattleEvent {
            title: "Event".to_owned(),
            source: "Default".to_owned(),
            target: "Default".to_owned(),
            effect: "No Effect".to_owned(),
        }
    }
}