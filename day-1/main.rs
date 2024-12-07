use std::{collections::HashMap, error::Error, fs::File, io::Read};

fn solve_part_2(input: String) -> Result<String, Box<dyn Error>> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for i in input.split("\n") {
        let (n1, n2) = i.split_once("   ").unwrap();
        list1.push(n1.parse::<i64>().unwrap());
        list2.push(n2.parse::<i64>().unwrap());
    }

    let frequencies = list2.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1i64);
        map
    });

    let mut total_dst = 0;
    for n in list1 {
        total_dst += n * frequencies.get(&n).unwrap_or(&0);
    }

    return Ok(format!("{:?}", total_dst));
}

fn solve_part_1(input: String) -> Result<String, Box<dyn Error>> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for i in input.split("\n") {
        let (n1, n2) = i.split_once("   ").unwrap();
        list1.push(n1.parse::<i64>().unwrap());
        list2.push(n2.parse::<i64>().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let mut total_dst = 0;
    for i in 0..list1.len() {
        let dst = (list2[i] - list1[i]).abs();
        total_dst += dst;
    }

    return Ok(format!("{:?}", total_dst));
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
