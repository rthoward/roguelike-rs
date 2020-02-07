pub struct GameState {
    running: bool,
    maps: Vec<Map>,
    map: usize,
}

impl GameState {
    pub fn get_map(&self) -> &Map {
        self.maps.get(self.map).expect("Could not get current map")
    }

    pub fn get_map_mut(&mut self) -> &mut Map {
        self.maps
            .get_mut(self.map)
            .expect("Could not get current map")
    }
}
