mod day_1;
mod day_2;

fn main() {
    let calibration_factor = day_1::get_calibration_factor();
    println!("Calibration factor: {:?}", calibration_factor);

    let id_sum = day_2::get_id_sum();
    println!("Game ID sum: {:?}", id_sum);
}

