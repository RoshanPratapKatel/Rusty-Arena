use crate::models::{Warrior, Goblin, Fighter, LootBox};
use super::ui; // We talk to the UI sibling

struct BattleReport<'a> {
    winner_name: &'a str,
    prize: String,
}

pub struct Game {
    pub player_one: Warrior,
    pub player_two: Goblin,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_one: Warrior {
                name: String::from("Roshan"),
                health: 100,
                attack_power: 15,
            },
            player_two: Goblin {
                name: String::from("Gobbo"),
                health: 80,
                attack_power: 10,
            },
        }
    }

    pub fn start(&mut self) {
        // Part 1: Print Intro and Player Entries
        ui::print_intro();
        ui::print_player_entry(&self.player_one.name(), self.player_one.hp());
        ui::print_player_entry(&self.player_two.name(), self.player_two.hp());
        ui::print_battle_start();

        // Part 2: Battle Loop
        loop {

            if self.player_one.hp() <= 0 {
                ui::print_defeat(&self.player_one.name());
                break;
            }

            let choice: i32 = ui::prompt_player_move();
            
            if choice == 1 {
                
                println!("You chose to Attack! ⚔️");
                
                self.player_two.receive_damage(self.player_one.attack_power);
                
                ui::log_attack(&self.player_one.name(), &self.player_two.name());
            
            } else {
                
                println!("You chose to Heal! 🩹");
                
                self.player_one.heal(10 as i32);
                
                println!("✨ {} healed up to {} HP!", self.player_one.name(), self.player_one.health);
            
            }

            if self.player_two.hp() <= 0 {
                ui::print_defeat(&self.player_two.name());
                break;
            }
            
            self.player_one.receive_damage(self.player_two.attack_power);
            ui::log_attack(&self.player_two.name(), &self.player_one.name());
        }

        // Part 3: Post-Battle Loot and Report
        let gold_box = LootBox { contents: 50 };
        ui::print_loot(&self.player_one.name(), gold_box.contents, "Gold Coins");
        let sword_box = LootBox { contents: String::from("Mighty Sword") };
        ui::print_loot(&self.player_one.name(), 0, &sword_box.contents);

        // Part 4: Battle Report
        let report: BattleReport<'_> = BattleReport {
            winner_name: &self.player_one.name(),
            prize: sword_box.contents,
        };
        ui::print_report(report.winner_name, &report.prize);
        
    }

}