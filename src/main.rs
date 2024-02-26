use core::fmt;
use std::io;
use rand::prelude::*;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
struct Player {
    name: String, 
    health: u32,
    strength: u32,
    gold: u32,
}

impl Player {
    fn display_stats(&self) {
        println!("{}| ♥ Health: {} 💰 Gold: {} 💪 Strength: {}", self.name, self.health, self.gold, self.strength);
    }

    fn take_damage_amount(&mut self, amount: u32) {
        self.health -= amount;
    }

    fn take_damage_rand(&mut self, min_dmg: u32, max_dmg: u32) {
        let mut rng: ThreadRng = rand::thread_rng();
        let damage: u32 = rng.gen_range(min_dmg..(max_dmg+1));
        print!("Damage Taken: {}", damage);
        self.health -= damage;
    }

    fn heal_for(&mut self, heal_amount: u32) {
        let heal_cost: u32 = heal_amount * 10;

        if self.gold < heal_cost {
            println!("Not enough gold to heal!");
            println!("This heal costs {} gold.", heal_cost);
        } else {
            if self.health < 100 {
                self.health += heal_amount; 
                self.gold -= heal_cost;
                if self.health > 100 {
                    self.health = 100;
                }
            } else {
                println!("Player is at max health")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Move, 
    Inventory, 
    Heal,
    Exit,
    DamageTest
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Turn::Move => "Move",
            Turn::Inventory => "Inventory",
            Turn::Heal => "Heal",
            Turn::Exit => "Exit",
            Turn::DamageTest => "DamageTest",
        })
    }
}


impl Turn {
    fn variants() -> Vec<Turn> {
        vec![
            Turn::Move, 
            Turn::Inventory,
            Turn::Heal,
            Turn::Exit,
            Turn::DamageTest,
        ]
    }
}

enum Enemy {

}

fn main() {
    let mut name_input: String = String::new();

    // Create a New Player and Get Name Based on User Input
    println!("Hello, welcome to the CLI Game. Please choose a name");
    io::stdin().read_line(&mut name_input).unwrap();
    let mut active_player: Player = dbg!(create_player(name_input.trim_end().to_string()));


    // While Player is "alive" request and execute turns
    while active_player.health > 0 {

      active_player.display_stats();

      let turn_selection: Turn = request_turn();

      run_turn(turn_selection, &mut active_player);
    } 

    println!("Your Player Has Died!");

}

fn create_player(name: String) -> Player {
    Player {
        name,
        health: 100,
        strength: 5,
        gold: 200
    }
}

/// Requests next Turn from player 
fn request_turn() -> Turn {

    let turn_options = Turn::variants();

    let turn_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Your Next Action: ")
        .items(&turn_options)
        .default(0) 
        .interact()
        .unwrap();

    turn_options[turn_selection]

}


/// Runs a Turn based off user selection for the given player
fn run_turn(turn: Turn, current_player: &mut Player) {
    match turn {
        Turn::Heal => {
            current_player.heal_for(5)
        }
        Turn::Move => {
            println!("Moving!")
        }
        Turn::Inventory => {
            println!("Checking Inventory!")
        }
        Turn::Exit => {
            println!("Exiting Game!")
        }
        Turn::DamageTest => {
            println!("Taking 10 Damage!");
            current_player.take_damage_amount(10);
        }
    }

}


