use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input/day_1.txt")?;
    let reader = BufReader::new(f);

    let mut calibration_sum = 0;
    for line in reader.lines() {
        calibration_sum += extract_calibration_value(line?);
    }
    println!("{calibration_sum}");
    Ok(())
}

fn extract_calibration_value(line: String) -> i32 {
    let mut first_digit = 'a';
    let mut second_digit = 'a';

    for c in line.chars() {
        if c.is_digit(10) {
            first_digit = c;
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            second_digit = c;
            break;
        }
    }

    let combined_value = format!("{first_digit}{second_digit}");
    combined_value.parse::<i32>().unwrap()
}

