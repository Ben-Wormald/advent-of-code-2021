pub fn solve_part_one(input: &str) -> u32 {
    let depths: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut increase_count = 0;

    depths
        .into_iter()
        .reduce(|prev_depth, depth| {
            if depth > prev_depth { increase_count += 1 };
            depth
        });

    increase_count
}

pub fn solve(input: &str) -> u32 {
    let depths: Vec<usize> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut increase_count = 0;

    depths
        .windows(3)
        .reduce(|prev_depth, depth| {
            if depth.into_iter().sum::<usize>() > prev_depth.into_iter().sum() {
                increase_count += 1;
            }
            depth
        });

    increase_count
}
