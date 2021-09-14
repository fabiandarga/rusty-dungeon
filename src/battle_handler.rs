use std::sync::Mutex;
use std::sync::Arc;
use crate::models::ActorValues;
use crate::models::models::Character;
use crate::models::Enemy;

pub struct BattleHandler {
    actors: Vec<Arc<Mutex<ActorValues>>>,
    sorted_actors: Vec<Arc<Mutex<ActorValues>>>,
    current_actor_index: usize,
}

impl BattleHandler {
    pub fn new() -> BattleHandler {
        BattleHandler {
            actors: Vec::new(),
            sorted_actors: Vec::new(),
            current_actor_index: 0,
        }
    }

    pub fn add_enemy(mut self, enemy: &Enemy) -> BattleHandler {
        self.actors.push(Arc::new(Mutex::new(enemy.into())));
        self
    }

    pub fn add_player(mut self, player: &Character) -> BattleHandler {
        self.actors.push(Arc::new(Mutex::new(player.into())));
        self
    }

    pub fn calc_initiative(&mut self) {
        self.actors.iter_mut().for_each(|actor| {
            let mut actor = actor.lock().unwrap();
            actor.initiative = Some(actor.agil)
        });
        self.sorted_actors = self.get_actor_values_by_order();
    }

    pub fn get_actor_values_by_order(&self) -> Vec<Arc<Mutex<ActorValues>>> {
        let mut copy = self.actors.clone();
        copy.sort_by(|a, b| {
            b.lock().unwrap().initiative.cmp(&a.lock().unwrap().initiative)
        });
        copy
    }

    pub fn get_current_actor(&self) -> Option<Arc<Mutex<ActorValues>>>{
        let opt = self.sorted_actors.get(self.current_actor_index);
        match opt {
            Some(arc) => Some(arc.clone()),
            None => None,
        }
    }

    pub fn increase_actor_index(&mut self) {
        self.current_actor_index += 1;
        if self.current_actor_index >= self.sorted_actors.len() {
            self.current_actor_index += 0;
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::models::models::Character;
    use super::*;

    #[test]
    fn test_add_enemy() {
        let enemy = Enemy::new(1, "Peter");
        let handler: BattleHandler = BattleHandler::new().add_enemy(&enemy);
        assert_eq!(handler.actors.len(), 1);
    }

    #[test]
    fn test_add_player() {
        let character = Character::default();

        let handler: BattleHandler = BattleHandler::new().add_player(&character);
        assert_eq!(handler.actors.len(), 1);
    }

    #[test]
    fn test_get_initiative() {
        let mut character = Character::default();
        character.agil = 10;

        let mut enemy_1 = Enemy::new(1, "Peter");
        enemy_1.agil = 5;

        let mut enemy_2 = Enemy::new(2, "Paul");
        enemy_2.agil = 1;
        
        let mut handler: BattleHandler = BattleHandler::new()
            .add_player(&character)
            .add_enemy(&enemy_1)
            .add_enemy(&enemy_2);
        handler.calc_initiative();

        let ordered = handler.get_actor_values_by_order();
        assert_eq!(ordered.len(), 3, "amount of actors");

        let nones: Vec<bool> = ordered.iter()
            .map(|x| x.lock().unwrap().initiative.is_none())
            .filter(|x| *x )
            .collect();
        assert_eq!(nones.len(), 0, "amount of actors without init");

        assert_eq!(ordered.get(0).unwrap().lock().unwrap().name, character.name, "first is the player");
    }
}