trait Fighter {
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
    fn receive_damage(&mut self, amount: i32);
}

struct Warrior {
    name: String,
    health: i32,
    attack_power: i32,
}

// Implementing the Fighter trait for Warrior struct
impl Fighter for Warrior {
    fn name(&self) -> &str {
        &self.name
    }

    fn hp(&self) -> i32 {
        self.health
    }

    fn receive_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
}

struct Goblin {
    name: String,
    health: i32,
    attack_power: i32,
}

impl Fighter for Goblin {
    fn name(&self) -> &str {
        &self.name
    }

    fn hp(&self) -> i32 {
        self.health
    }

    fn receive_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
}

// LootBox struct to hold generic contents
struct LootBox<T> {
    contents: T,
}

// ⚠️ DANGER ZONE
struct BattleReport<'a> {
    winner_name: &'a str,
    prize: String,
}

fn main() {
    println!("Welcome to the Rusty Arena! ⚔️");
    println!("Its going to be a fierce battle! Its a bumpy night ahead! Fasten your seatbelt 🌑");
    println!("==============================");

    let mut player_one: Warrior = Warrior {
        name: String::from("Roshan"),
        health: 100,
        attack_power: 15,
    };

    let mut player_two: Goblin = Goblin {
        name: String::from("Gobbo"),
        health: 80,
        attack_power: 10,
    };

    println!(
        "{} has entered the arena with {} HP!",
        player_one.name(),
        player_one.hp()
    );
    println!("==============================");

    println!(
        "{} has entered the arena with {} HP!",
        player_two.name(),
        player_two.hp()
    );
    println!("==============================");
    println!("==============================");

    println!("Now let the blood spill! 🩸");
    println!("==============================");
    print!("1 ... 2 ... 3 ... Bell Rings! 🔔\n\n");

    loop {
        // add a check if the player punching has hp > 0 before attacking
        if player_two.hp() <= 0 {
            println!("{} has been defeated! ☠️ ", player_two.name());
            break;
        }

        player_one.receive_damage(player_two.attack_power);
        println!("{} 🥊 {}", player_one.name(), player_two.name());
        
        if player_one.hp() <= 0 {
            println!("{} has been defeated! ☠️ ", player_one.name());
            break;
        }
        
        player_two.receive_damage(player_one.attack_power);
        println!("{} 🥊 {}", player_two.name(), player_one.name());
    }

    let gold_box: LootBox<i32> = LootBox { contents: 1000 };
    let sword_box: LootBox<String> = LootBox { contents: String::from("Excalibur") };

    println!(
        "{} found a loot box with {} gold coins! 💰",
        player_one.name(),
        gold_box.contents
    );
    println!(
        "{} found a loot box with a mighty sword: {}! 🗡️",
        player_one.name(),
        sword_box.contents
    );

    let report: BattleReport<'_> = BattleReport {
        winner_name: player_one.name(),
        prize: sword_box.contents,
    };
    println!(
        "🏆 The battle is over! The winner is {} who claimed the prize: {}! 🏆",
        report.winner_name, report.prize
    );
}
