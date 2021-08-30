use crate::models::models::Character;
use crate::models::models::Ability;
use rand::prelude::*;

const DEFAULT_N6_TRESHOLD: u8 = 4;

pub fn ability_check_with_nd6 (character: Character, ability: Ability, dificulty: u8) -> bool {
    let check = |score: u16, dif: u8| -> bool {
        let res = roll_nd6_against(score, DEFAULT_N6_TRESHOLD);
        let success = res.1 >= dif.into();
        success
    };
    ability_check(character, ability, dificulty, check)
}

fn ability_check<R>(character: Character, ability: Ability, dificulty: u8, check_fn: R ) -> bool 
where 
    R: Fn(u16, u8) -> bool,
{
    let score = match ability {
        Ability::Strg => character.strg,
        Ability::Agil => character.agil,
        Ability::Def => character.def,
    };

    check_fn(score, dificulty)
}


pub fn roll_nd6_against(rolls: u16, treshold: u8) -> (u16, u16) {
    let mut rng = thread_rng();
    let mut throws = 0;
    let mut successes: u16 = 0;
    let mut fails: u16 = 0;
    
    while throws < rolls {
        let res = rng.gen_range(1..6);
        match res {
            r if r >= treshold => {
                successes += 1;
            }
            _ => {
                fails += 1;
            }
        }
        throws += 1;
    }

    (successes, fails)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_roll() {
        let rolls = 5;
        let res = roll_nd6_against(rolls, 4);
        let results = res.0 + res.1;
        assert_eq!(results, rolls);
    }

    #[test]
    fn test_guaranteed_fails() {
        let rolls = 5;
        let res = roll_nd6_against(rolls, 7);
        assert_eq!(res.0, 0);
        assert_eq!(res.1, rolls);
    }

    #[test]
    fn test_guaranteed_successes() {
        let rolls = 3;
        let res = roll_nd6_against(rolls, 1);
        assert_eq!(res.0, rolls);
        assert_eq!(res.1, 0);
    }

    #[test]
    fn test_check_fn() {
        let mut character = Character::default();
        character.strg = 2;

        let abil = Ability::Strg;

        let dificulty = 2;

        let check = |score: u16, dif: u8| -> bool {
            return score >= dif.into();
        };

        let res = ability_check(character, abil, dificulty, check);
        assert_eq!(res, true);
    }
}