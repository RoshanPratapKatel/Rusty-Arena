use std::time::Duration;
use crossterm::event::KeyCode;
use crate::models::Warrior;
use crate::game::{ui, input, action};

pub struct Game {
    pub warrior: Warrior
}

impl Game {
    pub fn new() -> Self {
        Game {
            warrior: Warrior::new(50, 30),
        }
    }

    pub fn start(&mut self) {
        ui::init();

        loop {
            ui::draw(&self.warrior);

            if let Some(key) = input::read_key(Duration::from_millis(50)) {
                match key {
                    KeyCode::Esc => break, // Exit the game loop on Escape key
                    KeyCode::Left => action::move_warrior_left(&mut self.warrior.x),
                    KeyCode::Right => action::move_warrior_right(&mut self.warrior.x),
                    _ => {}
                }
            }
        }

        ui::cleanup();
    }
    

}