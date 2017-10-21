pub mod entity;
pub mod map;

#[derive(Debug)]
pub struct World {
    pub map: map::Map,
    pub entities: entity::Entities,
}
