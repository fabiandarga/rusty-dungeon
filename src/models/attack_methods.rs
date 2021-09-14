
use crate::models::BattleEvent;
use crate::models::ActorValues;

pub type AttackFunction = dyn Fn(&mut ActorValues, &mut ActorValues) -> BattleEvent;