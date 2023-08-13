use crate::character::Character;
use crate::{attack::*, TRIGGER_MIN, Z_TRIGGER};
use approx::*;

#[test]
fn test_knockback() {
    let fox = Character::Fox.get_stats();
    // marth's tipper fsmash
    // trajectory: 361.0
    let kb = knockback(
        20.0, 20.0, 70, 80, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    assert_relative_eq!(kb, 215.8);

    // falco shine
    // trajectory: 84.0
    let kb = knockback(
        8.0, 8.0, 50, 110, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );

    assert_relative_eq!(kb, 154.2);
}

#[test]
fn test_shield_stun() {
    // the values for these are equal parts manually tested and not.
    // i'm not sure how uncle punch displays shield stun, but i don't think it quite lines up with how
    // the community defines it or how it works intuitively? I'm not sure. But UP's values are universally 1 higher than
    // these, maybe because the last frame of hitlag and the first frame of hitstun decrement at the same time?
    let ss = shield_stun(17.0, 1.0, false);
    assert_eq!(ss, 9);

    let ss = shield_stun(12.0, Z_TRIGGER, false);
    assert_eq!(ss, 18);

    // tests weird animation shortening
    let ss = shield_stun(20.0, 1.0, false);
    assert_eq!(ss, 10);
}

#[test]
fn test_hitstun() {
    let fox = Character::Fox.get_stats();
    // marth's tipper fsmash
    // trajectory: 361.0
    let kb = knockback(
        20.0, 20.0, 70, 80, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    let hs = hitstun(kb);

    assert_eq!(hs, 86);

    // falco shine
    // trajectory: 84.0
    let kb = knockback(
        8.0, 8.0, 50, 110, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    let hs = hitstun(kb);

    assert_eq!(hs, 61);
}
