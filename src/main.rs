use std::path::Path;
use crate::utils::Error;

mod day_1;
mod day_2;
mod day_3;
mod utils;

fn main() -> Result<u32, Error> {
    let calibration_factor = day_1::get_calibration_factor();
    println!("Calibration factor: {:?}", calibration_factor);

    let id_sum = day_2::get_id_sum();
    println!("Game ID sum: {:?}", id_sum);

    let power = day_2::get_total_power();
    println!("Total power: {:?}", power);

    println!("Part number sum: {:?}",
             day_3::part_number_sum(utils::get_lines_from_file(Path::new("input/day_3.txt"))?));

    Ok(0)
}

