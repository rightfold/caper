use std::collections::HashMap;
use std::fmt;

use cgmath::Vector2;

pub const SECTOR_SIZE: usize = 8;

#[derive(Debug)]
pub struct Map {
    pub sectors: HashMap<Vector2<i32>, Sector>,
}

impl Map {
    pub fn new() -> Self {
        Map{
            sectors: HashMap::new(),
        }
    }
}

pub struct Sector {
    pub tiles: [Tile; SECTOR_SIZE * SECTOR_SIZE],
}

impl Sector {
    pub fn new() -> Self {
        let tiles = [Tile::Stone; SECTOR_SIZE * SECTOR_SIZE];
        Sector{tiles}
    }
}

impl fmt::Debug for Sector {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Sector {{ tiles: {:?} }}", &self.tiles[..])
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Stone,
}

pub mod graphics;
