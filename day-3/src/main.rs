use regex::Regex;
use std::{fs::File, io::Read};

/**
 * There are two new instructions you'll need to handle:
 * The do() instruction enables future mul instructions.
 * The don't() instruction disables future mul instructions.
 * At the beginning of the program, mul instructions are enabled.
 * Only the most recent do() or don't() instruction applies.
 *
 * Summary:
 * Same as before but order now matters.
 * Search for sequences matching do(), don't().
 * If do(): enable until we see a don't();
 *
 * mul is enabled at the start
 *
 * Can be a simple boolean.
 * Need to write a proper parser now
 */

struct TryMulResult {
    mul_result: i64,
    match_len: usize,
}

/**
 * Try the sequence as a mul instruction.
 */
fn try_mul_instruction(instruction: &str) -> Option<TryMulResult> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mul_result = match pattern
        .captures(&instruction)
        .and_then(|x| Some(x.extract()))
    {
        Some((_, [x, y])) => x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap(),
        None => return None,
    };

    Some({
        TryMulResult {
            mul_result,
            match_len: pattern.find(instruction).unwrap().len(),
        }
    })
}

fn solve_part_2(input: &str) -> i64 {
    let chars: Vec<char> = input.chars().collect();
    let mut search_buffer = String::with_capacity(chars.len());
    let mut mul_enabled = true;
    let mut total = 0;
    let mut cursor = 0;
    // mul(111,111)
    const MUL_INSTRUCTION_LEN: usize = 12usize;

    while cursor < chars.len() {
        search_buffer.push(chars[cursor]);
        println!("{}", search_buffer);
        if search_buffer.contains("do()") {
            search_buffer.truncate(0);
            mul_enabled = true;
        }

        if search_buffer.contains("don't()") {
            search_buffer.truncate(0);
            mul_enabled = false;
        }

        if search_buffer.contains("mul(") {
            // If multiply is not enabled, skip trying it, truncate the search buffer and continue
            if !mul_enabled {
                cursor += 1;
                search_buffer.truncate(0);
                continue;
            }

            let mut end_cursor = cursor + MUL_INSTRUCTION_LEN;
            // Clamp the cursor to the end of the string
            if end_cursor > chars.len() {
                end_cursor = chars.len();
            }
            // Do a lookahead for a mul instruction
            let mul_instruction_search_buffer: String =
                (&chars[(cursor - 4)..end_cursor]).iter().copied().collect();
            println!("Mul start: {}", mul_instruction_search_buffer);

            match try_mul_instruction(&mul_instruction_search_buffer) {
                Some(r) => {
                    println!("Found mul: {:?}", r.mul_result);
                    // If mul is enabled then add it to the result

                    total += r.mul_result;

                    // Skip the cursor ahead of the match length (minus the 4 chars of "mul(")
                    cursor += r.match_len - 4;
                    search_buffer.truncate(0);
                }
                None => {
                    // If the search failed, then reset the buffer but continue from
                    // here in case there was a do() or don't() instruction
                }
            }
            search_buffer.truncate(0);
        }

        cursor += 1;
    }

    total
}

/**
 * It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers.
 *
 * Summary:
 * Find sequences that match the regex sequence:
 * mul\(\d{1,3},\d{1,3}\)
 *
 * For each sequence, extract X & Y.
 * Multiply X & Y
 * Add the results up
 *
 * Approach:
 *
 * Could use regex as a naive approach.
 * A proper tokenizer could help in part 2 but for now regex seems easy enough.
 */
fn solve_part_1(input: String) -> i64 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0i64;
    for (_, [x, y]) in pattern.captures_iter(&input).map(|c| c.extract()) {
        total += x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap();
    }
    total
}

fn main() -> Result<(), std::io::Error> {
    let mut test_input: String = String::new();
    File::open("input")?
        .read_to_string(&mut test_input)
        .unwrap();

    let result1 = solve_part_1(test_input.trim().to_string());
    let result2 = solve_part_2(test_input.trim());

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
