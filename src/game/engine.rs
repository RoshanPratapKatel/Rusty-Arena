use std::time::Duration;
use crossterm::event::KeyCode;
use crate::models::{Warrior, Goblin};
use crate::game::{ui, input};
use crate::game::action::{move_warrior_left, move_warrior_right, move_goblin_down};

pub struct Game {
    pub warrior: Warrior,
    pub goblin: Goblin,
}

impl Game {
    pub fn new() -> Self {
        Game {
            warrior: Warrior::new(50, 30),
            goblin: Goblin::new(10, 0),
        }
    }

    pub fn start(&mut self) {
        ui::init();

        loop {
            ui::draw(&self.warrior, &self.goblin);

            if let Some(key) = input::read_key(Duration::from_millis(500)) {
                match key {
                    KeyCode::Esc => break, // Exit the game loop on Escape key
                    KeyCode::Left => move_warrior_left(&mut self.warrior.x),
                    KeyCode::Right => move_warrior_right(&mut self.warrior.x),
                    _ => {}
                }
            }

            move_goblin_down(&mut self.goblin.y);
            
        }

        ui::cleanup();
    }
    

}