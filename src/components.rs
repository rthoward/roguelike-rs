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

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Move { coord: Coord },
    Collision { collider: Entity, collidee: Entity },
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct EventsComponent {
    pub queue: Vec<Event>,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PlayerComponent;
