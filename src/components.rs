use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub asset_name: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Bar {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Brick {
    pub health: u8,
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: u8,
}

#[derive(Component, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
