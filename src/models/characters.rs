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

    fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > 100 {
            self.health = 100;
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

    fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > 80 {
            self.health = 80;
        }
    }
}

// Unit tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heal_test_for_warrior_caps_at_100() {
        let mut warriror = Warrior {
            name: String::from("TestWarrior"),
            health: 90,
            attack_power: 10,
        };
        warriror.heal(15);
        assert_eq!(warriror.health, 100);
    }

    #[test]
    fn heal_test_for_goblin_increases_health() {
        let mut goblin = Goblin {
            name: String::from("TestGoblin"),
            health: 60,
            attack_power: 8,
        };
        goblin.heal(15);
        assert_eq!(goblin.health, 75);
    }   
}