mod day_1;
mod day_2;
mod day_3;
mod utils;

fn main() {
    let calibration_factor = day_1::get_calibration_factor();
    println!("Calibration factor: {:?}", calibration_factor);

    let id_sum = day_2::get_id_sum();
    println!("Game ID sum: {:?}", id_sum);

    let power = day_2::get_total_power();
    println!("Total power: {:?}", power);

    let part_number_sum = day_3::part_number_sum();
    println!("Part number sum: {:?}", part_number_sum);
}

