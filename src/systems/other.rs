use crate::coord::Coord;
use specs::{prelude::*, Component};
use specs::{Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage};

use std::fmt;
use tcod::colors;

use super::movement::*;

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

pub struct EventDrain;
impl<'a> System<'a> for EventDrain {
    type SystemData = (
        WriteStorage<'a, MovementComponent>,
        WriteStorage<'a, CollisionComponent>,
        WriteStorage<'a, FighterComponent>,
        ReadStorage<'a, LabelComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut movement, mut collision, mut fighter, labels, _) = data;

        for movement in (&mut movement).join() {
            for _ in movement.events.drain(..) {}
        }

        for collision in (&mut collision).join() {
            for _ in collision.events.drain(..) {}
        }

        for fighter in (&mut fighter).join() {
            for f in fighter.events.drain(..) {
                let attacker_label = labels.get(f.attacker).expect("no attacker label");
                let attackee_label = labels.get(f.attackee).expect("no attackee label");
                println!("{} attacks {}", attacker_label, attackee_label);
            }
        }
    }
}

pub struct BasicAiSystem;
impl<'a> System<'a> for BasicAiSystem {
    type SystemData = (
        WriteStorage<'a, MovementComponent>,
        WriteStorage<'a, FighterComponent>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, BasicAiComponent>,
        Entities<'a>,
        specs::ReadExpect<'a, Entity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut movements, mut fighters, positions, basic_ai, entities, player) = data;
        let player_pos: Option<&PositionComponent> = positions.get(*player);

        for (movement, position, _, entity) in
            (&mut movements, &positions, &basic_ai, &entities).join()
        {
            if let Some(player_pos) = player_pos {
                if position.coord.distance(&player_pos.coord) <= 1 {
                    if let Some(fighter) = fighters.get_mut(entity) {
                        fighter.events.push(CombatEvent {
                            attacker: entity,
                            attackee: *player,
                        });
                    }
                } else {
                    movement.events.push(MoveEvent {
                        coord: position.coord.toward(&player_pos.coord).normalize(),
                    })
                }
            }
        }
    }
}

pub struct CombatSystem;
impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        WriteStorage<'a, FighterComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (fighters, _) = data;

        for fighter in (fighters).join() {
            for event in fighter.events.iter() {
                dbg!(event);
            }
        }
    }
}
