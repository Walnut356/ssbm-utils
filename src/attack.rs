#![allow(clippy::too_many_arguments)]

use std::{cmp, f32::consts::PI};

use crate::{
    character::*, BATTLEFIELD_BLASTZONES, DREAMLAND_BLASTZONES, FD_BLASTZONES, FOUNTAIN_BLASTZONES,
    STADIUM_BLASTZONES, YOSHIS_BLASTZONES,
};

#[cfg(test)]
mod test;

pub fn knockback(
    damage_staled: f32,
    damage_unstaled: f32,
    kb_growth: u32,
    base_kb: u32,
    set_kb: u32,
    is_throw: bool,
    character: &Attributes,
    percent: f32,
    crouch_cancel: bool,
    charge_interrupt: bool,
    vcancel: bool,
    metal: bool,
    ice: bool,
    dj_armor: bool,
) -> f32 {
    let weight: u32 = match is_throw {
        true => 100,
        false => character.weight,
    };

    let mut kb: f32;

    if set_kb == 0 {
        kb = (0.01 * kb_growth as f32)
            * ((1.4
                * (((0.05 * (damage_unstaled * (damage_staled + percent.floor())))
                    + (damage_staled + percent.floor()) * 0.1)
                    * (2.0 - (2.0 * (weight as f32 * 0.01)) / (1.0 + (weight as f32 * 0.01)))))
                + 18.0)
            + base_kb as f32;
    } else {
        kb = ((((set_kb * 10 / 20) + 1) as f32 * 1.4 * (200 / (weight + 100)) as f32 + 18.0)
            * (kb_growth / 100) as f32)
            + base_kb as f32;
    }

    if crouch_cancel {
        kb *= 0.667;
    }
    if charge_interrupt {
        kb *= 1.2;
    }
    if vcancel {
        kb *= 0.95;
    }
    if ice {
        kb *= 0.25;
    }
    if dj_armor {
        kb = f32::max(0.0, kb - 120.0);
    }
    if metal {
        kb = f32::max(0.0, kb - 30.0);
    }
    if character.name == "Nana" {
        kb = f32::max(0.0, kb - 5.0);
    }
    kb = f32::min(2500.0, kb);
    // if (trajectory > 180.0 && trajectory != 361.0) && grounded {
    //     if kb >= 80.0 {
    //         groundDownHitType = "Fly";
    //     } else {
    //         groundDownHitType = "Stay";
    //     }
    //     groundDownHit = true;
    // }

    kb
}

/// Used when recieving downwards knockback while grounded
pub enum GroundHitType {
    /// For knockback values higher than 80, causes character to bounce off the ground in tumble
    Fly,
    /// For knockback values 80 or lower, causes character to reel, but does not bounce or knock down
    Stay,
}

pub fn get_ground_hit_type(knockback: f32, trajectory: f32) -> Option<GroundHitType> {
    if trajectory > 180.0 && trajectory != 361.0 {
        if knockback > 80.0 {
            Some(GroundHitType::Fly)
        } else {
            Some(GroundHitType::Stay)
        }
    } else {
        None
    }
}

pub fn shield_stun(damage: f32, analog: f32, is_yoshi: bool) -> u32 {
    if is_yoshi {
        return 0;
    }
    let analog_scalar = 0.65 * (1.0 - (analog - 0.3) / 0.7);
    let shield_stun = (damage.floor() * (analog_scalar + 0.3) * 1.5 + 2.0) * (200.0 / 201.0);

    shield_stun.floor() as u32
}

// pub fn shield_size(shield_health: f32, analog: f32) -> f32 {
//     let analog_scalar = 1.0 - (0.5 * (analog - 0.3) / 0.7);
//     (shield_health * analog_scalar / 60.0) * 0.85 + 0.15
// }

pub fn shield_damage(damage: f32, analog: f32, powershield: bool) -> f32 {
    let mut ps_scalar: f32 = 1.0;
    if powershield {
        ps_scalar = 0.0;
    }

    let analog_scalar = 0.2 * (1.0 - (analog - 0.3) / 0.7);
    damage.floor() * (analog_scalar + 0.7) * ps_scalar
}

/// Calculates the hitlag for a given move.
///
/// electric modifier does not affect shield hits
pub fn hitlag(damage: f32, electric: bool, crouch_cancel: bool) -> u32 {
    let e: f32 = match electric {
        true => 1.5,
        false => 1.0,
    };
    let cc: f32 = match crouch_cancel {
        true => 2.0 / 3.0,
        false => 1.0,
    };
    cmp::min(
        ((((damage / 3.0).floor() + 3.0).floor() * e).floor() * cc).floor() as u32,
        20,
    )
}

pub fn hitstun(knockback: f32) -> u32 {
    (knockback * 0.4).floor() as u32
}

pub fn shield_pushback_defender(
    damage: f32,
    analog: f32,
    powershield: bool,
    is_yoshi: bool,
) -> f32 {
    let ps_scalar: f32 = {
        if powershield {
            1.0
        } else {
            0.6
        }
    };

    let d_push: f32 = {
        if is_yoshi {
            let a = 0.3 * (1.0 - (analog - 0.3) / 0.7);
            (damage.floor() * a) + 0.14
        } else {
            // only non-yoshi defender is capped
            let a = 0.195 * (1.0 - (analog - 0.3) / 0.7);
            ((damage.floor() * (a + 0.09) + 0.4) * ps_scalar).clamp(0.0, 2.0)
        }
    };

    d_push
}

pub fn shield_pushback_attacker(damage: f32, analog: f32) -> f32 {
    let a = (analog - 0.3) * 0.1;
    (damage.floor() * a) + 0.02
}

pub fn knockback_travel(
    knockback: f32,
    trajectory: f32,
    hitstun: u32,
    position_x: f32,
    position_y: f32,
) -> Vec<(f32, f32)> {
    let mut result = Vec::with_capacity(hitstun as usize + 1);
    result.push((position_x, position_y));

    result
}

pub fn get_horizontal_decay(angle: f32) -> f32 {
    0.051 * (angle * (PI / 180.0)).cos()
}

//Rate at which vertical velocity decreases
//Gravity also plays a role, but that is done in knockbackTravel
pub fn get_vertical_decay(angle: f32) -> f32 {
    0.051 * (angle * (PI / 180.0)).sin()
}

pub fn will_tumble(kb: f32) -> bool {
    kb > 80.0
}

/// Accepts a tournament legal stage's in-game id and a pair of X, Y coordinates. Returns true if the player is dead
pub fn is_past_blastzone(stage: u16, position_x: f32, position_y: f32) -> bool {
    let blast_zones = match stage {
        2 => FOUNTAIN_BLASTZONES,
        3 => STADIUM_BLASTZONES,
        8 => YOSHIS_BLASTZONES,
        28 => DREAMLAND_BLASTZONES,
        31 => BATTLEFIELD_BLASTZONES,
        32 => FD_BLASTZONES,
        _ => panic!("invalid stage ID"),
    };
    use crate::BlastZone::*;

    !(position_x < blast_zones[Right as usize]
        && position_x > blast_zones[Left as usize]
        && position_y < blast_zones[Top as usize]
        && position_y > blast_zones[Bottom as usize])
}
