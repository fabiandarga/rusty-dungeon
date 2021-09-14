use crate::data::WithId;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Encounter {
    pub id: u16,
    pub text: String,
    pub enemies: Vec<EnemyDeclaration>
}

impl WithId for Encounter {
    fn get_id(&self) -> u16 {
        self.id
    }
}

#[derive(Clone, Deserialize)]
pub struct EnemyDeclaration(u16, EnemyAmount);

#[derive(Clone, Deserialize)]
pub enum EnemyAmount {
    Amount(u8),
    Range(u8, u8),
}