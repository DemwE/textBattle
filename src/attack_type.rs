use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum AttackType {
    Physical,
    Magic,
}