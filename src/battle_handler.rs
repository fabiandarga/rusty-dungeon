use crate::models::ActorValues;
use crate::models::models::Character;
use crate::models::Enemy;

pub struct BattleHandler {
    pub enemies: Vec<Enemy>,
    pub player: Option<Character>,
    pub actors: Vec<ActorValues>,
}

impl BattleHandler {
    pub fn new() -> BattleHandler {
        BattleHandler {
            enemies: Vec::new(),
            actors: Vec::new(),
            player: None,
        }
    }

    pub fn add_enemy(mut self, enemy: &Enemy) -> BattleHandler {
        self.actors.push(enemy.into());
        self
    }

    pub fn add_player(mut self, player: &Character) -> BattleHandler {
        self.actors.push(player.into());
        self
    }

    pub fn calc_initiative(&mut self) {
        self.actors.iter_mut().for_each(|actor| actor.initiative = Some(actor.agil));
    }

    pub fn get_actor_values_by_order(&self) -> Vec<ActorValues> {
        let mut copy = self.actors.clone();
        copy.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        copy
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
        let character = Character::default();
        let enemy_1 = Enemy::new(1, "Peter");
        let enemy_2 = Enemy::new(2, "Paul");
        let mut handler: BattleHandler = BattleHandler::new()
            .add_player(&character)
            .add_enemy(&enemy_1)
            .add_enemy(&enemy_2);
        handler.calc_initiative();

        let ordered = handler.get_actor_values_by_order();
        assert_eq!(ordered.len(), 3, "amount of actors");

        let nones: Vec<bool> = ordered.iter()
            .map(|x| x.initiative.is_none())
            .filter(|x| *x )
            .collect();
        assert_eq!(nones.len(), 0, "amount of actors without init");
    }
}