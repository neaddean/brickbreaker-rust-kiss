pub enum EntityType {
    Ball { x: f32, y: f32, r: f32},
    Brick { x: f32, y: f32, health: u8 },
    Bar,
}

