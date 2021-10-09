// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const PRODUCTION_PER_HOUR :f64 = 221.0;
const ONE_TO_FOUR_DEFECT_RATE : f64= 1.0;
const FIVE_TO_EIGHT_DEFECT_RATE: f64 = 0.9;
const NINE_TO_TEN_DEFECT_RATE: f64 = 0.77;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    

    match speed {
        0..=4 => {
            (speed as f64 * PRODUCTION_PER_HOUR as f64 * ONE_TO_FOUR_DEFECT_RATE)
        },
        5..=8 => {
            (speed as f64 * PRODUCTION_PER_HOUR as f64 * FIVE_TO_EIGHT_DEFECT_RATE)
        },
        9..= 10 => {
            (speed as f64 * PRODUCTION_PER_HOUR as f64 * NINE_TO_TEN_DEFECT_RATE)
        },
        _ => {
            0.0
        }
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
        production_rate_per_hour(speed) as u32 / 60
    }

