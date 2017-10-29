use cgmath::Vector2;

pub trait MonsterSet {
    type Id: Copy;
    fn ids(&self) -> &[Self::Id];
    fn positions(&self) -> &[Vector2<f32>];
    fn healths(&self) -> &[f32];
    fn despawn(&mut self, Self::Id);
}

pub fn despawn_dead<MS: MonsterSet>(monsters: &mut MS) {
    let dead = {
        let ids = monsters.ids().iter();
        let healths = monsters.healths().iter();
        ids.zip(healths)
            .filter(|&(_, &health)| health < 0.0)
            .map(|(&id, _)| id)
            .collect::<Vec<_>>()
    };
    for dead in dead {
        monsters.despawn(dead);
    }
}

pub mod catalog;
pub mod collections;
