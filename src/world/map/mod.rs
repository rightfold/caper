use std::collections::HashMap;
use std::fmt;

use cgmath::Vector2;

use graphics::gl;

pub const SECTOR_SIZE: usize = 32;

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
    pub materials:  [Material; SECTOR_SIZE * SECTOR_SIZE],
    pub elevations: [u8;       SECTOR_SIZE * SECTOR_SIZE],
}

impl Sector {
    pub fn new() -> Self {
        let mut materials = [Material::Stone; SECTOR_SIZE * SECTOR_SIZE];
        let elevations = [0; SECTOR_SIZE * SECTOR_SIZE];
        for i in 0 .. SECTOR_SIZE {
            materials[i] = Material::Grass;
        }
        Sector{materials, elevations}
    }
}

impl fmt::Debug for Sector {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Sector {{ materials: {:?}, elevations: {:?} }}",
               &self.materials[..], &self.elevations[..])
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Material {
    Stone = 0x00,
    Grass = 0x01,
}

unsafe impl gl::ArrayComponentData for Material {
    type Target = gl::ArrayComponentDataTargetInteger;

    fn component_count() -> usize {
        1
    }

    fn component_data_type() -> gl::ArrayComponentDataType {
        gl::ArrayComponentDataType::UnsignedShort
    }
}

unsafe impl gl::BufferData for Material { }

pub mod graphics;
