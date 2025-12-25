// Entry point for the library
pub mod fighters;
pub mod characters;
pub mod items;

// Re-exporting for easier access
pub use fighters::Fighter;
pub use characters::{Warrior, Goblin};
pub use items::LootBox;