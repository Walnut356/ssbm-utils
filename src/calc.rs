use std::cmp;

pub fn shield_stun(damage: &f32, analog: &f32) -> i32 {
    let a: f32 = 0.65 * (1.0 - (analog - 0.3) / 0.7);
    return (((damage * (a + 0.3)) * 1.5 + 2.0) * (200.0 / 201.0)).floor() as i32;
}

pub fn shield_size(hp: &f32, analog: &f32) -> f32 {
    let a = 1.0 - (0.5 * (analog - 0.3) / 0.7);
    return (hp * a / 60.0) * 0.85 + 0.15;
}

pub fn hit_lag(damage: &f32, electric: Option<bool>, crouch_cancel: Option<bool>) -> i32 {
    let e: f32 = match electric.unwrap_or(false) {
        true => 1.5,
        false => 1.0,
    };
    let cc: f32 = match crouch_cancel.unwrap_or(false) {
        true => 0.666667,
        false => 1.0,
    };
    return cmp::min(
        ((((damage / 3.0).floor() + 3.0).floor() * e).floor() * cc).floor() as i32,
        20,
    );
}

pub fn shield_pushback(damage: &f32, analog: &f32) -> Vec<f32> {
    //todo yoshi
    //defender
    let mut a = 0.195 * (1.0 - (analog - 0.3) / 0.7);
    let d_push = (damage * (a + 0.09) + 0.4) * 0.6; //TODO .6 for normal, 1 for power

    //attacker
    a = (analog - 0.3) * 0.1;
    let a_push = (damage * a) + 0.02;
    return vec![d_push, a_push];
}
