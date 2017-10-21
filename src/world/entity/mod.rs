#[derive(Debug)]
pub struct Entities {
    pub spiders: spider::SpiderSet,
}

pub mod collections;
pub mod spider;
