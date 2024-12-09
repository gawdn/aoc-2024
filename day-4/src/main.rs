use std::{fs::File, io::Read};

/**
 * It's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X.
 * Summary:
 *
 * Find all distinct occurrences of MAS overlapping in an X shape. MAS may be written backwards or forwards.
 *
 * Case 1:
 * M.S
 * .A.
 * M.S
 * Case 2:
 * S.S
 * .A.
 * M.M
 * Case 3:
 * M.M
 * .A.
 * S.S
 * Case 4:
 * S.M
 * .A.
 * S.M
 *
 * This is a convolution problem.
 * For each character, assume it is the top left of the one of the four variants:
 * - Case 1: MS
 * - Case 2: SS
 * - Case 3: MM
 * - Case 4: SM
 *
 * Check the kernel to see if it matches any of the cases.
 *
 * Optimizations could include:
 * - Skip if the character is X since it obviously can't be that
 * - Only do searches for the M/S variant based on what the letter is
 *
 * Maybe can do some fancy bitmasking but probably overoptimising.
 */

fn solve_part_2(input: String) -> i64 {
    let search_grid = parse_word_search(input);
    let mut total = 0;
    // We don't check the last two rows because the kernel can't fit in the remaining space.
    for row in 0..(search_grid.len() - 2) {
        // Don't check last two columns either. Assuming search rows are of uniform size.
        for col in 0..(search_grid.first().unwrap().len() - 2) {
            total += if is_valid_cross_pattern(&search_grid, row, col) {
                println!("Found cross at {row},{col}");

                1
            } else {
                0
            }
        }
    }
    total
}

fn is_valid_cross_pattern(
    input: &Vec<Vec<char>>,
    starting_row: usize,
    starting_col: usize,
) -> bool {
    // Hardcoded diagonals to check
    let top_left_to_bottom_right: String = vec![
        input[starting_row][starting_col],
        input[starting_row + 1][starting_col + 1],
        input[starting_row + 2][starting_col + 2],
    ]
    .into_iter()
    .collect();
    let top_right_to_bottom_left: String = vec![
        input[starting_row][starting_col + 2],
        input[starting_row + 1][starting_col + 1],
        input[starting_row + 2][starting_col],
    ]
    .into_iter()
    .collect();

    // They must both be valid
    (top_left_to_bottom_right == "SAM" || top_left_to_bottom_right == "MAS")
        && (top_right_to_bottom_left == "SAM" || top_right_to_bottom_left == "MAS")
}

type SearchLine = Vec<char>;
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
