use specs::{
    world::{World, WorldExt},
    Builder,
};
use tcod::colors;

use crate::components::*;
use crate::coord::Coord;

pub fn player(world: &mut World, coord: Coord) {
    world
        .create_entity()
        .with(PositionComponent { coord, map: 0 })
        .with(RenderComponent {
            glyph: '@',
            fg: colors::WHITE,
            bg: None,
        })
        .with(MovementComponent { events: vec![] })
        .with(CollisionComponent { events: vec![] })
        .with(PlayerComponent)
        .build();
}

pub fn orc(world: &mut World, coord: Coord) {
    world
        .create_entity()
        .with(PositionComponent { coord, map: 0 })
        .with(RenderComponent {
            glyph: 'o',
            fg: colors::GREEN,
            bg: None,
        })
        .with(CollisionComponent::default())
        .build();
}
