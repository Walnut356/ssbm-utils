use ssbm_utils::calc::*;
fn main() {
    let damage = 9.0;
    let analog = 1.0;
    let thing = hit_lag(&damage, Some(false), Some(false));
    let eef = shield_stun(&damage, &analog);
    let freef = shield_pushback(&damage, &analog);
    println!("{thing}");
    println!("{eef}");
    println!("{:?}", freef);
}