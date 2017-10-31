use world::effect::damage_indicator::DamageIndicatorSet;

const MAX_AGE: f32 = 1.0;

pub fn simulate(damage_indicators: &mut DamageIndicatorSet, dt: f32) {
    simulate_aging(damage_indicators, dt);
    despawn_too_old(damage_indicators);
}

fn simulate_aging(damage_indicators: &mut DamageIndicatorSet, dt: f32) {
    let ages = soa_array!(damage_indicators, mut ages).iter_mut();
    for age in ages {
        *age += dt;
    }
}

fn despawn_too_old(damage_indicators: &mut DamageIndicatorSet) {
    let too_old = {
        let ids = damage_indicators.ids().iter();
        let ages = soa_array!(damage_indicators, ages).iter();
        ids.zip(ages)
            .filter(|&(_, &age)| age > MAX_AGE)
            .map(|(&id, _)| id)
            .collect::<Vec<_>>()
    };
    for too_old in too_old {
        damage_indicators.despawn(too_old);
    }
}
