#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Transform {
    pub position: Position,
    pub scale: Scale,
}