use std::{error::Error, fs::File, io::Read};

fn solve_part_2(input: String) -> Result<String, Box<dyn Error>> {
    return Ok(input.to_string());
}

fn solve_part_1(input: String) -> Result<String, Box<dyn Error>> {
    return Ok(input.to_string());
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
