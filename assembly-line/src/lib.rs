// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0..=4 => return speed as f64* 221 as f64,
        5..=8 => return speed as f64 * 0.9 * 221 as f64,
        9..=10 => return speed as f64 * 0.77 * 221 as f64,
        _ => return 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    (production_rate_per_hour(speed) / 60.0) as u32
}
fn main(){
    println!("{}",production_rate_per_hour(4));
}