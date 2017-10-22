use std::collections::HashMap;
use std::fmt;

pub const CHUNK_SIZE: usize = 16;

#[derive(Debug)]
pub struct Map {
    pub chunks: HashMap<(isize, isize), Chunk>,
}

impl Map {
    pub fn new() -> Self {
        Map{
            chunks: HashMap::new(),
        }
    }
}

pub struct Chunk {
    pub tiles: [Tile; CHUNK_SIZE * CHUNK_SIZE],
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Chunk {{ tiles: {:?} }}", &self.tiles[..])
    }
}

#[derive(Debug)]
pub enum Tile {

}
