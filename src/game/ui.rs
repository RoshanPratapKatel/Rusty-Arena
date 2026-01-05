use std::io::{stdout, Write};
use crossterm::{
    cursor,
    execute,
    terminal::{self, Clear, ClearType},
    style::{Print, Color, SetForegroundColor, ResetColor},
};
use crate::models::{Goblin, Warrior};


pub fn init() {
    // Switch to the alternate screen buffer. This ensures the game draws on a 
    // fresh canvas and preserves the user's terminal history upon exit.
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    
    // Enable raw mode to capture input directly. Without this, terminal
    // input would be line-buffered (this means input is sent only after Enter is pressed).
    terminal::enable_raw_mode().unwrap();
}


pub fn draw(warrior: &Warrior, goblin: &Goblin) {
    // stdout() constructs a new handle to the standard output each time it's called. 
    let mut stdout = stdout();

    // Clear the screen - we want to redraw everything from scratch
    // This is a simple approach for demonstration purposes. In a real game, you'd want to optimize this.
    execute!(stdout, Clear(ClearType::All)).unwrap();

    // Move cursor to warrior position 
    execute!(stdout, cursor::MoveTo(warrior.x, warrior.y)).unwrap();

    // Draw the warrior in Green color
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("@"),
        ResetColor
    ).unwrap();

    // Move cursor to goblin position 
    execute!(stdout, cursor::MoveTo(goblin.x, goblin.y)).unwrap();

    // Draw the goblin in Red color
    execute!(
        stdout,
        SetForegroundColor(Color::Red),
        Print("G"),
        ResetColor
    ).unwrap();

    // By default, the handle is line-buffered, meaning it flushes only when a newline is printed.
    // But in our case, we want to see the updates immediately, so we manually flush the buffer.
    stdout.flush().unwrap();
}


pub fn cleanup() {
    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}