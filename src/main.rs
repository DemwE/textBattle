mod character;
pub mod armor;
pub mod weapon;
mod attack_type;

use std::fs;
use character::Character;

fn main() {
    let hero_json = fs::read_to_string("./src/hero.json").expect("Unable to read file");
    let mut hero: Character = serde_json::from_str(&hero_json).expect("JSON was not well-formatted");
    println!("Hero: {:?}", hero);

    let enemy_json = fs::read_to_string("./src/enemy.json").expect("Unable to read file");
    let mut enemy: Character = serde_json::from_str(&enemy_json).expect("JSON was not well-formatted");
    println!("Enemy: {:?}", enemy);

    //Hero stats
    println!("{} health {}", hero.name(), hero.health());

    //Enemy stats
    println!("{} health {}", enemy.name(), enemy.health());

    // Battle loop
    while hero.health() > 0.0 && enemy.health() > 0.0 {
        // Hero attacks enemy
        hero.attack(&mut enemy);
        println!("{} attacks. {} health is now {}", hero.name(), enemy.name(), enemy.health());

        // Check if enemy is defeated
        if enemy.health() <= 0.0 {
            println!("{} wins!", hero.name());
            break;
        }

        // Enemy attacks hero
        enemy.attack(&mut hero);
        println!("{} attacks. {} health is now {}", enemy.name(), hero.name(), hero.health());

        // Check if hero is defeated
        if hero.health() <= 0.0 {
            println!("{} wins!", enemy.name());
            break;
        }
    }
}
