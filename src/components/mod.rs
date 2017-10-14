use specs::*;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position(pub f32, pub f32);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity(pub f32, pub f32);