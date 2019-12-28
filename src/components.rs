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
pub enum Event {
    Move {
        entity: Entity,
        coord: Coord,
    },
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct EventsComponent {
  pub actions: Vec<Event>,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PlayerComponent;
