use std::{collections::HashMap, error::Error, fs::File, io::Read};
fn diff_signs(report: Vec<i64>) -> Vec<i64> {
    let mut signs = vec![];
    for i in 1..report.len() {
        let sign = (report[i] - report[i - 1]).signum();
        signs.push(sign);
    }
    signs
}

fn solve_part_2(input: String) -> Result<String, Box<dyn Error>> {
    let mut safe_report_count = 0;
    return Ok(safe_report_count.to_string());
}

fn solve_part_1(input: String) -> Result<String, Box<dyn Error>> {
    let mut reports = vec![];
    for r in input.split("\n") {
        reports.push(
            r.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }

    let mut safe_report_count = 0;
    for r in reports {
        let mut prev_value = r[0];
        let mut safe = true;
        let mut prev_sign = 0;

        for n in &r[1..] {
            let diff = n - prev_value;
            let abs_diff = diff.abs();
            let curr_sign = diff.signum();
            if curr_sign == 0 || (prev_sign != 0 && curr_sign != prev_sign) {
                // Changed direction or stayed the same
                safe = false;
                break;
            }

            if abs_diff < 1 || abs_diff > 3 {
                safe = false;
                break;
            }

            prev_sign = curr_sign;
            prev_value = *n;
        }

        safe_report_count += if safe { 1 } else { 0 };
    }

    return Ok(safe_report_count.to_string());
}

fn main() -> Result<(), std::io::Error> {
    let mut test_input: String = String::new();
    File::open("input")?
        .read_to_string(&mut test_input)
        .unwrap();

    let result1 = solve_part_1(test_input.trim().to_string());
    let result2 = solve_part_2(test_input.trim().to_string());

    println!("Part 1: {}", result1.unwrap());
    println!("Part 2: {}", result2.unwrap());

    Ok(())
}
