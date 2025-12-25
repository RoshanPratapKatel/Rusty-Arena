
pub fn print_intro() {
    println!("Welcome to the Rusty Arena! ⚔️");
    println!("Its going to be a fierce battle! Its a bumpy night ahead! Fasten your seatbelt 🌑");
    println!("==============================");
}

pub fn print_player_entry(name: &str, hp: i32) {
    println!(
        "{} has entered the arena with {} HP!",
        name,
        hp
    );
    println!("==============================");
}

pub fn print_battle_start() {
    println!("==============================");
    println!("Now let the blood spill! 🩸");
    println!("==============================");
    print!("1 ... 2 ... 3 ... Bell Rings! 🔔\n\n");
}

pub fn log_attack(attacker: &str, defender: &str) {
    println!("{} 🥊 {}", attacker, defender);
}

pub fn print_defeat(name: &str) {
    println!("{} has been defeated! ☠️ ", name);
}

pub fn print_loot(player_name: &str, gold: i32, item: &str) {
    println!("{} found a loot box with {} gold coins! 💰", player_name, gold);
    println!("{} found a loot box with a mighty sword: {}! 🗡️", player_name, item);
}

pub fn print_report(winner: &str, prize: &str) {
    println!(
        "🏆 The battle is over! The winner is {} who claimed the prize: {}! 🏆",
        winner, prize
    );
}