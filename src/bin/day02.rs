fn is_safe_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let increasing = numbers[1] > numbers[0];
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if (increasing && diff < 0) || (!increasing && diff > 0) {
            return false;
        }
    }

    true
}

fn is_safe_with_one_bad(numbers: &[i32]) -> bool {
    if is_safe_sequence(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut tmp: Vec<i32> = numbers[..i].to_vec();
        tmp.extend_from_slice(&numbers[i + 1..]);

        if is_safe_sequence(&tmp) {
            return true;
        }
    }

    false
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            is_safe_sequence(&numbers)
        })
        .count()
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            is_safe_with_one_bad(&numbers)
        })
        .count()
}

fn main() {
    let input =
        std::fs::read_to_string("input/day02.txt").expect("should have been able to read the file");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}
