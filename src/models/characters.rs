use super::fighters::Fighter; 

pub struct Warrior {
    pub name: String,
    pub health: i32,
    pub attack_power: i32,
}


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

pub struct Goblin {
    pub name: String,
    pub health: i32,
    pub attack_power: i32,
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