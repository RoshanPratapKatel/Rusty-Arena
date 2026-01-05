
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

pub fn move_goblin_down(y: &mut u16) {
    if *y < 30 {
        *y += 1;
    } else {
        *y = 0;
    }
}