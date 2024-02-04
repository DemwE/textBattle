use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::attack_type::AttackType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Armor {
    pub helmet: Option<Helmet>,
    pub chestplate: Option<Chestplate>,
    pub leggings: Option<Leggings>,
}

impl Armor {
    pub fn new(helmet: Option<Helmet>, chestplate: Option<Chestplate>, leggings: Option<Leggings>) -> Armor {
        Armor {
            helmet,
            chestplate,
            leggings,
        }
    }

    pub fn calculate_total_defense(&self) -> f32 {
        let helmet_defense = self.helmet.as_ref().map_or(0.0, |h| h.defense);
        let chestplate_defense = self.chestplate.as_ref().map_or(0.0, |c| c.defense);
        let leggings_defense = self.leggings.as_ref().map_or(0.0, |l| l.defense);
        helmet_defense + chestplate_defense + leggings_defense
    }

    fn update_resistances<T: ArmorPiece>(total_resistances: &mut HashMap<AttackType, f32>, armor_piece: &T) {
        for (attack_type, resistance) in armor_piece.resistance() {
            *total_resistances.entry(attack_type.clone()).or_insert(0.0) += resistance;
        }
    }

    pub fn get_defense_types(&self) -> HashMap<AttackType, f32> {
        let mut total_resistances: HashMap<AttackType, f32> = HashMap::new();

        if let Some(armor_piece) = &self.helmet {
            Self::update_resistances(&mut total_resistances, armor_piece)
        }

        if let Some(armor_piece) = &self.chestplate {
            Self::update_resistances(&mut total_resistances, armor_piece)
        }

        if let Some(armor_piece) = &self.leggings {
            Self::update_resistances(&mut total_resistances, armor_piece)
        }

        total_resistances
    }
}

pub trait ArmorPiece {
    fn resistance(&self) -> &HashMap<AttackType, f32>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Helmet {
    pub name: String,
    pub defense: f32,
    pub resistance: HashMap<AttackType, f32>,
}

impl ArmorPiece for Helmet {
    fn resistance(&self) -> &HashMap<AttackType, f32> {
        &self.resistance
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chestplate {
    pub name: String,
    pub defense: f32,
    pub resistance: HashMap<AttackType, f32>,
}

impl ArmorPiece for Chestplate {
    fn resistance(&self) -> &HashMap<AttackType, f32> {
        &self.resistance
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leggings {
    pub name: String,
    pub defense: f32,
    pub resistance: HashMap<AttackType, f32>,
}

impl ArmorPiece for Leggings {
    fn resistance(&self) -> &HashMap<AttackType, f32> {
        &self.resistance
    }
}