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

fn main() {
    println!("The Arena is under construction ...");
    
    // Instantiate a Warrior player
    let player_one: Warrior = Warrior {
        name: String::from("Roshan"),
        health: 100,
        attack_power: 15,
    };

    let player_two: Goblin = Goblin {
        name: String::from("Gobbo"),
        health: 80,
        attack_power: 10,
    };

    println!(
        "{} has entered the arena with {} HP!",
        player_one.name(),
        player_one.hp()
    );

    println!(
        "{} has entered the arena with {} HP!",
        player_two.name(),
        player_two.hp()
    );
}
