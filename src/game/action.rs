
pub fn move_warrior_left(x: &mut u16) {
    if *x > 0 {
        *x -= 1;
    }
}

pub fn move_warrior_right(x: &mut u16) {
    if *x < 100 {
        *x += 1;
    }
}