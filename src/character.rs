use serde::{Serialize, Deserialize};
use crate::weapon::Weapon;
use crate::armor::Armor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    name: String,
    health: f32,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
}

impl Character {
    fn new(name: &str, health: f32, weapon: Weapon, armor: Option<Armor>) -> Character {
        Character {
            name: name.to_string(),
            health,
            weapon: Some(weapon),
            armor,
        }
    }

    fn calculate_damage(&self, other: &Character) -> f32 {
        let damage = self.weapon.as_ref().unwrap().damage();
        let attack_type = self.weapon.as_ref().unwrap().get_attack_type();
        let total_defense = other.armor.as_ref().unwrap().calculate_total_defense();
        let defense_type = other.armor.as_ref().unwrap().get_defense_types();

        let resistance = defense_type.get(attack_type).unwrap_or(&0.0);

        (damage - (total_defense * resistance)).max(0.0)
    }

    pub fn attack(&self, other: &mut Character) {
        let damage = self.calculate_damage(other);
        other.health -= damage;
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}