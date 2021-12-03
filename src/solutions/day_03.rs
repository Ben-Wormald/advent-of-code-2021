fn binary_to_decimal(binary: Vec<usize>) -> usize {
    let mut bit_value = 1;
    binary.into_iter().rev().fold(0, |sum, bit| {
        let sum = if bit == 0 { sum } else { sum + bit_value };
        bit_value *= 2;
        sum
    })
}

fn binary_str_to_decimal(binary: &str) -> usize {
    binary_to_decimal(
        binary.chars()
            .map(|bit| bit.to_string().parse().unwrap())
            .collect()
    )
}

pub fn solve_part_one(input: &str) -> usize {
    let bit_count = *&input.lines().next().unwrap().len();
    let mut one_counts = vec![0; bit_count];
    let mut total_lines = 0;
    
    input.lines()
        .for_each(|line| {
            line.chars()
                .enumerate()
                .for_each(|(pos, bit)|
                    if bit == '1' { one_counts[pos] += 1 }
                );
            total_lines += 1;
        });

    let threshold = total_lines / 2;
    let gamma_rate: Vec<usize> = one_counts.iter()
        .map(|count|
            if count > &threshold { 1 } else { 0 }
        )
        .collect();
    let gamma_rate = binary_to_decimal(gamma_rate);
    let epsilon_rate = 2usize.pow(bit_count as u32) - 1 - gamma_rate;

    gamma_rate * epsilon_rate
}

fn filter_lines<'a>(lines: &Vec<&'a str>, pos: usize, use_most_common: bool) -> Vec<&'a str> {
    let (mut zero_count, mut one_count) = (0, 0);

    lines.iter()
        .for_each(|line|
            if line.chars().nth(pos).unwrap() == '0' { zero_count += 1 } else { one_count += 1 }
        );

    let criterion = if use_most_common {
        if zero_count > one_count { '0' } else { '1' }
    } else {
        if zero_count > one_count { '1' } else { '0' }
    };

    lines.iter()
        .filter(|line|
            line.chars().nth(pos).unwrap() == criterion
        )
        .copied()
        .collect()
}

fn get_match<'a>(lines: &Vec<&'a str>, pos: usize, use_most_common: bool) -> &'a str {
    let filtered_lines = filter_lines(lines, pos, use_most_common);
    match filtered_lines.len() {
        len if len > 1 => get_match(&filtered_lines, pos + 1, use_most_common),
        1 => filtered_lines[0],
        _ => panic!("no matches!")
    }
}

pub fn solve(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let oxygen_rating = get_match(&lines, 0, true);
    let co2_rating = get_match(&lines, 0, false);

    let oxygen_rating = binary_str_to_decimal(oxygen_rating);
    let co2_rating = binary_str_to_decimal(co2_rating);

    oxygen_rating * co2_rating
}
