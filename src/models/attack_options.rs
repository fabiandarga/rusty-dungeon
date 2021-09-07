#[derive(Clone)]
pub struct AttackDescription {
    pub title: String,
    pub attack_type: String,
    pub dmg_min: usize,
    pub dmg_max: usize,
    pub special_effect: String,
}

#[derive(Clone)]
pub enum AttackOption {
    Attack(AttackDescription),
    None
}

pub type AttackOptions = Vec<AttackOption>;