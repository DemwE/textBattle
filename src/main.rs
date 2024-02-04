mod character;
mod armor;
mod weapon;
mod attack_type;

use std::fs;
use std::thread::sleep;
use character::Character;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;

fn main() {
    let hero_json = fs::read_to_string("./src/hero.json").expect("Unable to read file");
    let mut hero: Character = serde_json::from_str(&hero_json).expect("JSON was not well-formatted");
    println!("Hero: {:?}", hero);

    let enemy_json = fs::read_to_string("./src/enemy.json").expect("Unable to read file");
    let mut enemy: Character = serde_json::from_str(&enemy_json).expect("JSON was not well-formatted");
    println!("Enemy: {:?}", enemy);

    let m = Arc::new(MultiProgress::new());

    let hero_pb = m.add(ProgressBar::new(hero.health() as u64));
    hero_pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos}/{len}").unwrap());

    let enemy_pb = m.add(ProgressBar::new(enemy.health() as u64));
    enemy_pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40.red/yellow} {pos}/{len}").unwrap());

    while hero.health() > 0.0 && enemy.health() > 0.0 {
        hero.attack(&mut enemy);
        enemy_pb.set_position((enemy.health() as u64).max(0));

        if enemy.health() <= 0.0 {
            println!("{} wins!", hero.name());
            break;
        }

        enemy.attack(&mut hero);
        hero_pb.set_position((hero.health() as u64).max(0));

        if hero.health() <= 0.0 {
            println!("{} wins!", enemy.name());
            break;
        }

        sleep(std::time::Duration::from_millis(500));
    }

    if hero.health() <= 0.0 {
        println!("{} wins!", enemy.name());
    } else {
        println!("{} wins!", hero.name());
    }

    hero_pb.finish_and_clear();
    enemy_pb.finish_and_clear();
}
