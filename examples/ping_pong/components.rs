use skalora_game_engine::components::component::Component;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

pub struct Paddle {
    pub player: Player,
    pub speed: f32,
}

impl Component for Paddle {}

pub struct Ball;

impl Component for Ball {}
