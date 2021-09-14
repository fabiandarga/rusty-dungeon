pub struct Enemy {
    pub id: usize,
    pub name: String,
    pub hp: u16,
    pub hp_max: u16,
    pub strg: u16,
    pub agil: u16,
    pub def: u16,
}

impl Enemy {
    pub fn new(id: usize, name: &str) -> Enemy {
        Enemy { 
            id,
            name: name.to_owned(),
            hp: 0,
            hp_max: 0,
            strg: 0,
            agil: 0,
            def: 0,
        }
    }
}
