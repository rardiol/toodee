
use crate::*;

pub type TooDeeMap<T> = TooDee<Option<T>>;

impl<T> TooDeeMap<T> {
    pub fn map_values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.cells_mut().filter_map(|x|x.as_mut())
    }
    pub fn map_get(&self, coord: Coordinate) -> Option<&T> {
        self.get(coord).map(|x|x.as_ref()).flatten()
    }
    pub fn map_contains_key(&self, coord: Coordinate) -> bool {
        self.map_get(coord).is_some()
    }
}

impl<T: Clone> TooDeeMap<T> {
    pub fn map_insert(&mut self, coord: Coordinate, el: T) {
        self.insert(coord, Some(el))
    }
}
