mod days;
use days::*;
use std::fs;
use std::env;

use std::time::Instant;
use phf_macros::phf_map;
use chrono::{Utc, Datelike};

fn get_day_run(day: u32) -> fn(&String) -> () {
    type DayRun = fn(&String) -> ();
    static DAYRUNS: phf::Map<u32, DayRun> = phf_map! {
        1u32 => day_01::run,
        2u32 => day_02::run,
        3u32 => day_03::run,
        4u32 => day_04::run,
        5u32 => day_05::run,
        6u32 => day_06::run,
        7u32 => day_07::run,
        8u32 => day_08::run,
        9u32 => day_09::run,
        10u32 => day_10::run,
        11u32 => day_11::run,
        12u32 => day_12::run,
        13u32 => day_13::run,
        14u32 => day_14::run,
        15u32 => day_15::run,
        16u32 => day_16::run,
        17u32 => day_17::run,
        18u32 => day_18::run,
        19u32 => day_19::run,
        20u32 => day_20::run,
        21u32 => day_21::run,
        22u32 => day_22::run,
        23u32 => day_23::run,
        24u32 => day_24::run,
    };
    *DAYRUNS.get(&day).unwrap()
}

fn main() {
    let mut day = Utc::now().day();
    if env::args().len() == 2 {
        let args: Vec<String> = env::args().collect();
        day = args[1].parse().unwrap();
    }

    let input_file_name = format!("inputs/input_{:02}.txt", day);
    let input= fs::read_to_string(&input_file_name)
        .expect(&format!("Could not open the file {}", &input_file_name));

    let now = Instant::now();
    get_day_run(day)(&input);
    let duration = now.elapsed();
    dbg!(duration);
}
