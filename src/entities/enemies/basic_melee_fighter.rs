use crate::models::Actor;
use crate::models::BattleEvent;
use crate::models::attack_methods::AttackFunction;
use crate::models::ActorValues;

pub struct BasicMeleeFighter {
    pub values: ActorValues,
}

impl Actor for BasicMeleeFighter {
    fn get_actor_values(&self) -> ActorValues {
        self.values.clone()
    }
    fn selectAttack(&self) -> Box<AttackFunction> {
        Box::new(melee_attack)
    }
}

fn melee_attack(user: &mut ActorValues, target: &mut ActorValues) -> BattleEvent {
    let dmg = 0;

    BattleEvent {
        title: "Melee Attack".to_owned(),
        source: user.name.to_owned(),
        target: target.name.to_owned(),
        effect: format!("{} lost {} hp", target.name, dmg)
    }
}