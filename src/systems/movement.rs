#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct MovementComponent {
    pub events: Vec<MoveEvent>,
}

#[derive(Default)]
pub struct MovementSystem {
    occupied_coords: HashMap<Coord, Entity>,
}
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, PositionComponent>,
        WriteStorage<'a, MovementComponent>,
        WriteStorage<'a, CollisionComponent>,
        WriteStorage<'a, FighterComponent>,
        specs::ReadExpect<'a, GameState>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (mut positions, mut movements, mut collisions, mut fighters, game_state, entities) =
            data;
        let map = game_state.get_map();

        for (position, entity) in (&positions, &*entities).join() {
            self.occupied_coords.insert(position.coord, entity);
        }
        for (position, movements, entity) in (&mut positions, &mut movements, &*entities).join() {
            for MoveEvent { coord } in movements.events.iter() {
                let target_coord = position.coord.add(coord);
                if map.can_move(&target_coord) {
                    if let Some(collidee) = self.occupied_coords.get(&target_coord) {
                        // Collided with another entity. Attack them
                        if collisions.get(*collidee).is_some() && fighters.get(*collidee).is_some()
                        {
                            if let Some(fighter) = fighters.get_mut(entity) {
                                fighter.events.push(CombatEvent {
                                    attacker: entity,
                                    attackee: *collidee,
                                })
                            }
                        }
                    } else {
                        // No collisions. Move to the target coord
                        self.occupied_coords.remove(&position.coord);
                        position.coord = target_coord;
                        self.occupied_coords.insert(target_coord, entity);
                    }
                }
            }
        }
    }
}
