use crate::coord::Coord;
use specs::{prelude::*, Component};
use tcod::colors;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PositionComponent {
    pub coord: Coord,
    pub map: usize,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct RenderComponent {
    pub glyph: char,
    pub fg: colors::Color,
    pub bg: Option<colors::Color>,
}

#[derive(Debug)]
pub struct MoveEvent {
    pub coord: Coord,
}

#[derive(Debug)]
pub struct CollisionEvent {
    pub collider: Entity,
    pub collidee: Entity,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct MovementComponent {
    pub events: Vec<MoveEvent>,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CollisionComponent {
    pub events: Vec<CollisionEvent>,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PlayerComponent;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct FighterComponent {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct BasicAiComponent;
