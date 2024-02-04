use serde::{Serialize, Deserialize};
use crate::attack_type::AttackType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon {
    pub name: String,
    pub damage: f32,
    pub attack_type: AttackType,
}

impl Weapon {
    pub fn new(name: &str, damage: f32, attack_type: AttackType) -> Weapon {
        Weapon {
            name: name.to_string(),
            damage,
            attack_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub fn get_attack_type(&self) -> &AttackType {
        &self.attack_type
    }
}