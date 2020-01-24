use crate::coord::Coord;
use specs::{prelude::*, Component};
use std::fmt;
use tcod::colors;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct LabelComponent {
    pub label: String,
}

impl LabelComponent {
    pub fn new(label: &str) -> Self {
        LabelComponent {
            label: label.to_owned(),
        }
    }
}

impl fmt::Display for LabelComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

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

#[derive(Debug)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub attackee: Entity,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct FighterComponent {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub events: Vec<CombatEvent>,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct BasicAiComponent;
