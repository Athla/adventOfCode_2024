use std::fs;

fn part01(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
    ];

    let is_valid = |row: i32, col: i32| -> bool {
        row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
    };

    for start_row in 0..rows {
        for start_col in 0..cols {
            for (dx, dy) in directions.iter() {
                let mut valid = true;
                let word = "XMAS";

                for (i, expected_char) in word.chars().enumerate() {
                    let curr_row = start_row as i32 + (dx * i as i32);
                    let curr_col = start_col as i32 + (dy * i as i32);

                    if !is_valid(curr_row, curr_col)
                        || grid[curr_row as usize][curr_col as usize] != expected_char
                    {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1
                }
            }
        }
    }

    count
}
fn part02(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for center_row in 1..rows - 1 {
        for center_col in 1..cols - 1 {
            if grid[center_row][center_col] != 'A' {
                continue;
            }
            let positions = [
                (center_row - 1, center_col - 1),
                (center_row - 1, center_col + 1),
                (center_row + 1, center_col - 1),
                (center_row + 1, center_col + 1),
            ];

            for &top_left in ['M', 'S'].iter() {
                for &top_right in ['M', 'S'].iter() {
                    for &bottom_right in ['M', 'S'].iter() {
                        for &bottom_left in ['M', 'S'].iter() {
                            if is_valid_xmas(
                                top_left,
                                top_right,
                                bottom_left,
                                bottom_right,
                                &grid,
                                &positions,
                            ) {
                                count += 1
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn is_valid_xmas(
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    grid: &Vec<Vec<char>>,
    positions: &[(usize, usize)],
) -> bool {
    let actual_top_left = grid[positions[0].0][positions[0].1];
    let actual_top_right = grid[positions[1].0][positions[1].1];
    let actual_bottom_left = grid[positions[2].0][positions[2].1];
    let actual_bottom_right = grid[positions[3].0][positions[3].1];

    if actual_top_left != top_left
        || actual_top_right != top_right
        || actual_bottom_left != bottom_left
        || actual_bottom_right != bottom_right
    {
        return false;
    }

    let diagonal1_forward = top_left == 'M' && bottom_right == 'S'; // M->A->S
    let diagonal1_backward = top_left == 'S' && bottom_right == 'M'; // S->A->M
    let diagonal2_forward = top_right == 'M' && bottom_left == 'S'; // M->A->S
    let diagonal2_backward = top_right == 'S' && bottom_left == 'M'; // S->A->M

    // We need at least one valid sequence in each diagonal
    let diagonal1_valid = diagonal1_forward || diagonal1_backward;
    let diagonal2_valid = diagonal2_forward || diagonal2_backward;

    diagonal1_valid && diagonal2_valid
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("./input/day04.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 01: {}", part01(&input));
    println!("Part 02: {}", part02(&input));
}
