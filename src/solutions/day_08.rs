const UNIQUE_LENGTHS: [usize; 4] = [2, 3, 4, 7];

pub fn solve(input: &str) -> isize {
    input.lines()
        .fold(0, |sum, line|
            sum + line.split(" | ")
                .last().unwrap()
                .split_whitespace()
                .fold(0, |line_sum, output|
                    if UNIQUE_LENGTHS.contains(&output.len()) {
                        line_sum + 1
                    } else {
                        line_sum
                    }
                )
        )
}
