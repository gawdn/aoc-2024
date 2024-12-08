use std::{fs::File, io::Read};

type SearchLine = Vec<char>;
fn solve_part_2(input: String) -> i64 {
    0
}

/**
 * This word search allows words to be
 * horizontal,
 * vertical,
 * diagonal,
 * written backwards, or
 * even overlapping other words.
 *
 * Summary:
 * - Find the following lines:
 *   - Each row
 *   - Each column
 *   - Each diagonal >= 4 in length
 * - Add the reverse of each line
 * - count the number of XMAS
 */
fn solve_part_1(input: String) -> i64 {
    const WORD: &str = "XMAS";
    let search_grid = parse_word_search(input);
    let search_lines = get_all_search_lines(&search_grid, WORD);
    let mut total = 0;
    for line in search_lines {
        total += count_word_in_search_line(WORD, &line);
    }
    total
}

fn parse_word_search(input: String) -> Vec<Vec<char>> {
    let mut lines = vec![];
    for r in input.split("\n") {
        lines.push(r.chars().into_iter().collect());
    }
    lines
}

fn count_word_in_search_line(word: &str, line: &Vec<char>) -> i64 {
    let search_line: String = line.into_iter().cloned().collect();
    search_line.matches(word).count().try_into().unwrap()
}

fn tmp_dbg(target_debug: &SearchLine) {
    println!("{:?}", target_debug);
    println!("{:?}", count_word_in_search_line("XMAS", target_debug));
}

fn get_all_search_lines(input: &Vec<Vec<char>>, word: &str) -> Vec<SearchLine> {
    let mut all_search_lines = vec![];
    // Rows
    let rows: Vec<Vec<char>> = input.to_vec();
    all_search_lines.push(get_reverse_lines(&rows));
    all_search_lines.push(rows);

    // Columns
    let columns = get_columns(input);
    all_search_lines.push(get_reverse_lines(&columns));
    all_search_lines.push(columns);

    // Diagonals
    let diagonals = get_diagonals(input, word.len());
    all_search_lines.push(get_reverse_lines(&diagonals));
    all_search_lines.push(diagonals);

    all_search_lines.concat()
}

fn get_reverse_lines(input: &Vec<SearchLine>) -> Vec<SearchLine> {
    input
        .iter()
        .map(|l| l.into_iter().rev().cloned().collect())
        .collect()
}

fn get_columns(input: &Vec<Vec<char>>) -> Vec<SearchLine> {
    let mut columns: Vec<Vec<char>> = vec![];
    // Prepopulate the columns list with empty vecs, assuming rows are of
    // uniform size
    for _ in 0..input[0].len() {
        columns.push(vec![]);
    }
    for row_i in 0..input.len() {
        for col_i in 0..input[0].len() {
            columns[col_i].push(input[row_i][col_i]);
        }
    }
    columns
}

fn get_diagonals(input: &Vec<Vec<char>>, min_len: usize) -> Vec<SearchLine> {
    let mut diagonals: Vec<Vec<char>> = vec![];
    let rows = input.len();
    let cols = input[0].len();

    // Top-left to bottom-right diagonals
    for start_col in 0..cols {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = start_col;
        while row < rows && col < cols {
            diagonal.push(input[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal);
    }
    for start_row in 1..rows {
        let mut diagonal = Vec::new();
        let mut row = start_row;
        let mut col = 0;
        while row < rows && col < cols {
            diagonal.push(input[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal);
    }

    // Top-right to bottom-left diagonals
    for start_col in (0..cols).rev() {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = start_col;
        while row < rows && col < cols {
            diagonal.push(input[row][col]);
            row += 1;
            col = if col > 0 { col - 1 } else { break };
        }
        diagonals.push(diagonal);
    }
    for start_row in 1..rows {
        let mut diagonal = Vec::new();
        let mut row = start_row;
        let mut col = cols - 1;
        while row < rows && col < cols {
            diagonal.push(input[row][col]);
            row += 1;
            col = if col > 0 { col - 1 } else { break };
        }
        diagonals.push(diagonal);
    }
    diagonals
        .iter()
        .filter(|d| d.len() >= min_len)
        .cloned()
        .collect()
}

fn main() -> Result<(), std::io::Error> {
    let mut test_input: String = String::new();
    File::open("input")?
        .read_to_string(&mut test_input)
        .unwrap();

    let result1 = solve_part_1(test_input.trim().to_string());
    let result2 = solve_part_2(test_input.trim().to_string());

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
