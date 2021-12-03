fn binary_to_decimal(binary: Vec<usize>) -> usize {
    let mut bit_value = 1;
    binary.into_iter().rev().fold(0, |sum, bit| {
        let sum = if bit == 0 { sum } else { sum + bit_value };
        bit_value *= 2;
        sum
    })
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

fn filter_lines<'a>(lines: &Vec<&'a str>, criterion: char, pos: usize) -> Vec<&'a str> {
    lines.iter()
        .filter(|line|
            line.chars().nth(pos).unwrap() == criterion
        )
        .copied()
        .collect()
}

fn get_match<'a>(lines: &Vec<&'a str>, criteria: Vec<char>, pos: usize) -> &'a str {
    let filtered_lines = filter_lines(lines, criteria[pos], pos);
    match filtered_lines.len() {
        len if len > 1 => get_match(&filtered_lines, criteria, pos + 1),
        1 => filtered_lines[0],
        _ => panic!("no matches!")
    }
}

fn binary_str_to_decimal(binary: &str) -> usize {
    binary_to_decimal(
        binary.chars()
            .map(|bit| bit.to_string().parse().unwrap())
            .collect()
    )
}

pub fn solve(input: &str) -> usize {
    let bit_count = *&input.lines().next().unwrap().len();
    let mut one_counts = vec![0; bit_count];
    let mut total_lines = 0;

    let lines: Vec<&str> = input.lines().collect();
    
    lines.iter()
        .for_each(|line| {
            line.chars()
                .enumerate()
                .for_each(|(pos, digit)|
                    if digit == '1' { one_counts[pos] += 1 }
                );
            total_lines += 1;
        });

    let threshold = total_lines / 2;

    let oxygen_criteria: Vec<char> = one_counts.iter()
        .map(|count|
            if count >= &threshold { '1' } else { '0' }
        )
        .collect();

    let co2_criteria: Vec<char> = oxygen_criteria.iter()
        .map(|bit| if *bit == '0' { '1' } else { '0' })
        .collect();

    let oxygen_rating = get_match(&lines, oxygen_criteria, 0);
    let co2_rating = get_match(&lines, co2_criteria, 0);

    let oxygen_rating = binary_str_to_decimal(oxygen_rating);
    let co2_rating = binary_str_to_decimal(co2_rating);

    oxygen_rating * co2_rating
}
