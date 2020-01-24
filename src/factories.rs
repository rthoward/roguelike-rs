use specs::{
    world::{World, WorldExt},
    Builder, Entity,
};
use tcod::colors;

use crate::components::*;
use crate::coord::Coord;

pub fn player(world: &mut World, coord: Coord) -> Entity {
    world
        .create_entity()
        .with(LabelComponent::new("player"))
        .with(PositionComponent { coord, map: 0 })
        .with(RenderComponent {
            glyph: '@',
            fg: colors::WHITE,
            bg: None,
        })
        .with(MovementComponent::default())
        .with(CollisionComponent::default())
        .with(PlayerComponent)
        .with(FighterComponent {
            max_hp: 20,
            hp: 20,
            defense: 5,
            power: 5,
            events: vec![],
        })
        .build()
}

pub fn orc(world: &mut World, coord: Coord) -> Entity {
    world
        .create_entity()
        .with(LabelComponent::new("orc"))
        .with(PositionComponent { coord, map: 0 })
        .with(RenderComponent {
            glyph: 'o',
            fg: colors::GREEN,
            bg: None,
        })
        .with(MovementComponent::default())
        .with(CollisionComponent::default())
        .with(FighterComponent {
            max_hp: 10,
            hp: 10,
            defense: 3,
            power: 3,
            events: vec![],
        })
        .with(BasicAiComponent)
        .build()
}
