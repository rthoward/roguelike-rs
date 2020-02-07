mod collision;
mod movement;
mod other;
mod tcod;

use specs::{Entities, Entity, ReadStorage, System, World, WorldExt, WriteStorage, Dispatcher};
use other::*;
use movement::*;
use ::tcod::{TcodSystem};

pub fn register(world: World) -> World {
    world.register::<PositionComponent>();
    world.register::<RenderComponent>();
    world.register::<MovementComponent>();
    world.register::<CollisionComponent>();
    world.register::<PlayerComponent>();
    world.register::<FighterComponent>();
    world.register::<BasicAiComponent>();
    world.register::<LabelComponent>();

    world
}

pub fn setup_dispatcher<'a>(world: World, tcod: TcodSystem) -> Dispatcher<'a> {
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(tcod)
        .with(BasicAiSystem {}, "basic_ai", &[])
        .with(MovementSystem::default(), "movement_system", &["basic_ai"])
        .with(EventDrain {}, "event_drain", &["movement_system"])
        .with(CombatSystem {}, "combat_system", &["movement_system"])
        .build();

    dispatcher.setup(&mut world)
}
