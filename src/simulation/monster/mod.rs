use world::monster::{MonsterSet, Mortal};

pub fn despawn_dead<M>(monsters: &mut M)
    where M: MonsterSet + Mortal {
    let dead = {
        let ids     = monsters.ids().iter();
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

pub mod gas_spore;
pub mod spider;
