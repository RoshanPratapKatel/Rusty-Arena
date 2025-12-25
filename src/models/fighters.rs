pub trait Fighter {
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
    fn receive_damage(&mut self, amount: i32);
}
