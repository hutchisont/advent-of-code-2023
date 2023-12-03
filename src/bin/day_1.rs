use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input/day_1.txt")?;
    let reader = BufReader::new(f);

    let mut calibration_sum = 0;
    let mut line_count = 0;
    for line in reader.lines() {
        line_count += 1;
        let line = line?;
        println!("starting line: {line}");
        let line = convert_words_to_digits(line);
        println!("ending line: {line}");
        calibration_sum += extract_calibration_value(line);
    }
    println!("line count: {line_count}");
    println!("calibration sum: {calibration_sum}");
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

fn convert_words_to_digits(line: String) -> String {
    let mut digitized_line = String::new();

    for i in 0..line.len() {
        let reduced_line = &line[i..];
        let result = if reduced_line.starts_with("one") {
            "1"
        } else if reduced_line.starts_with("two") {
            "2"
        } else if reduced_line.starts_with("three") {
            "3"
        } else if reduced_line.starts_with("four") {
            "4"
        } else if reduced_line.starts_with("five") {
            "5"
        } else if reduced_line.starts_with("six") {
            "6"
        } else if reduced_line.starts_with("seven") {
            "7"
        } else if reduced_line.starts_with("eight") {
            "8"
        } else if reduced_line.starts_with("nine") {
            "9"
        } else {
            &line[i..i + 1]
        };
        digitized_line += result;
    }

    digitized_line
}
