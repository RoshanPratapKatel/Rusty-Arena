use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

// Returns Some(KeyCode) if a key was pressed within the timeout.
// Returns None if no key was pressed.
pub fn read_key(timeout: Duration) -> Option<KeyCode> {
    // 1. Poll: Check if there is an event waiting (non-blocking wait)
    if event::poll(timeout).ok()? {
        // 2. Read: If yes, grab it
        if let Event::Key(KeyEvent { code, .. }) = event::read().ok()? {
            return Some(code);
        }
    }
    None
}