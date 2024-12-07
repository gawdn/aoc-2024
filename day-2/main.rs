use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Read,
    vec,
};

/**
 *
 * SAFE:
 * In the array all the values are either all increasing or all decreasing.
 * In the array any two adjacent values differ by at least one and at most three.
 * If you remove 1 value from the array, both of the above are true.
 *
 * Summary:
 * Is a report safe?
 * For an array of positive ints, check if they are all decreasing or increasing.
 * For an array of positive ints, check if any two adjacent values differ by at least 1 & at most 3.
 * If both are true:
 * - SAFE
 * else:
 * If I make a new array A(n - 1) (A missing 1 element) from A(n).
 * If both of the original conditions are safe for this new A(n - 1) array:
 * - Also SAFE
 *
 * Naive solution is to just generate all arrays that are A(n-1)
 */

fn parse_reports(input: String) -> Vec<Vec<i64>> {
    let mut reports = vec![];
    for r in input.split("\n") {
        reports.push(
            r.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    reports
}

fn generate_all_n_minus_1_reports(report: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut reports = vec![];
    for i in 0..report.len() {
        let mut n_minus_1_report = report.clone();
        n_minus_1_report.remove(i);
        reports.push(n_minus_1_report);
    }
    reports
}

fn is_safe_report(report: &Vec<i64>) -> bool {
    let mut signs = vec![];

    // Check if the absolute differences are in range
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        // Absolute difference is not within range 1 <= x <= 3
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        let sign = diff.signum();
        // Difference is 0, therefore cannot be ascending or descending
        if sign == 0 {
            return false;
        }

        signs.push(diff.signum());
    }

    // Check if all the signs are the same
    return signs.iter().all(|x| *x == signs[0]);
}

fn is_safe_report_with_dampening(report: &Vec<i64>) -> bool {
    if is_safe_report(&report) {
        return true;
    } else {
        // Check all arrays n - 1
        let n_minus_1_reports = generate_all_n_minus_1_reports(&report);
        for n_minus_1_report in n_minus_1_reports {
            if is_safe_report(&n_minus_1_report) {
                return true;
            }
        }
    }

    return false;
}

fn solve_part_2(input: String) -> Result<String, Box<dyn Error>> {
    let reports = parse_reports(input);
    let mut safe_report_count = 0;

    for r in reports {
        safe_report_count += if is_safe_report_with_dampening(&r) {
            1
        } else {
            0
        }
    }

    return Ok(safe_report_count.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe_ascending() {
        let result = is_safe_report_with_dampening(&vec![1, 2, 3, 5]);
        assert!(result);
    }

    #[test]
    fn safe_descending() {
        let result = is_safe_report_with_dampening(&vec![1, 2, 3, 5]);
        assert!(result);
    }

    #[test]
    fn safe_with_dampening_ascending() {
        // If we remove 0; the sequence 1 3 5 7 still fulfils the contract
        let result = is_safe_report_with_dampening(&vec![1, 0, 3, 5, 7]);
        assert!(result);
    }

    #[test]
    fn safe_with_dampening_descending() {
        // If we remove 0; the sequence 7 5 3 1 still fulfils the contract
        let result = is_safe_report_with_dampening(&vec![7, 5, 0, 3, 1]);
        assert!(result);
    }

    #[test]
    fn safe_with_dampening_gap() {
        // If we remove 1; the sequence 5 7 8 fulfils the contract
        // Notably we can't remove 5 here because it won't fulfil the contract
        let result = is_safe_report_with_dampening(&vec![1, 5, 7, 8]);
        assert!(result)
    }

    #[test]
    fn unsafe_with_dampening_two_violations() {
        // This is unsafe because:
        // Either 0 is removed and the sequence fails to be either ascending or descending
        // Or 8 is removed and the sequence gap is too large betwen 0 - 5
        let result = is_safe_report_with_dampening(&vec![0, 8, 5, 7, 8]);
        assert!(!result)
    }
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
