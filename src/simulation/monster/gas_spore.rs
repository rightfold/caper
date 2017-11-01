use rand::Rng;

use simulation::wander;
use world::monster::gas_spore::*;

const ALTITUDE_CHANGE_SPEED: f32 = 0.6;

pub fn simulate<R: Rng>(gas_spores: &mut GasSporeSet, rng: &mut R, dt: f32) {
    wander::simulate_wander(rng, dt,
                            &mut gas_spores.scalars.since_target_changes,
                            soa_array!(gas_spores, mut angles),
                            soa_array!(gas_spores, mut positions),
                            soa_array!(gas_spores, mut targets),
                            soa_array!(gas_spores, mut velocities));

    simulate_altitude_arcsins(gas_spores, dt);
}

fn simulate_altitude_arcsins(gas_spores: &mut GasSporeSet, dt: f32) {
    let altitude_arcsins = soa_array!(gas_spores, mut altitude_arcsins).iter_mut();
    for altitude_arcsin in altitude_arcsins {
        *altitude_arcsin = simulate_altitude_arcsin(*altitude_arcsin, dt);
    }
}

fn simulate_altitude_arcsin(altitude_arcsin: f32, dt: f32) -> f32 {
    altitude_arcsin + ALTITUDE_CHANGE_SPEED * dt
}
