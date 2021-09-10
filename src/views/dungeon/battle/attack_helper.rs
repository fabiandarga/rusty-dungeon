pub fn build_damage_text(dmg_min: usize, dmg_max: usize) -> String {
    if dmg_min == 0 && dmg_max == 0 {
        return "No damage".to_owned();
    } else if dmg_max > 0 {
        return format!("{}-{} damage", dmg_min, dmg_max);
    } else {
        return format!("{} damage", dmg_min);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_damage_text_none() {
        let text = build_damage_text(0, 0);
        assert_eq!(text, "No damage");
    }

    #[test]
    fn test_damage_text() {
        let text = build_damage_text(5, 0);
        assert_eq!(text, "5 damage");
    }

    #[test]
    fn test_damage_text_range() {
        let text = build_damage_text(0, 5);
        assert_eq!(text, "0-5 damage");
    }
}