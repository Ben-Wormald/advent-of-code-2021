const UNIQUE_LENGTHS: [usize; 4] = [2, 3, 4, 7];

pub fn solve_part_one(input: &str) -> usize {
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

#[derive(Debug)]
struct Digit<'a> {
    example: &'a str,
    value: usize,
}

fn contains_all(digit_a: &str, digit_b: &str) -> bool {
    digit_b.chars().all(|part| digit_a.contains(part))
}

fn contains_n(digit_a: &str, digit_b: &str, n: usize) -> bool {
    digit_b.chars().filter(|part| digit_a.contains(*part)).count() == n
}

fn is_equal(digit_a: &str, digit_b: &str) -> bool {
    contains_all(digit_a, digit_b) && contains_all(digit_b, digit_a)
}

fn get_value(line: &str) -> usize {
    let parts: Vec<Vec<&str>> = line
        .split(" | ")
        .map(|part| part.split(" ").collect())
        .collect();

    let digits = &parts[0];
    let output_digits = &parts[1];
    let mut mappings: Vec<Digit> = vec!();

    let one = digits.iter().find(|digit| digit.len() == 2).unwrap();
    let seven = digits.iter().find(|digit| digit.len() == 3).unwrap();
    let four = digits.iter().find(|digit| digit.len() == 4).unwrap();
    let eight = digits.iter().find(|digit| digit.len() == 7).unwrap();

    let three = digits.iter().find(|digit|
        digit.len() == 5 && contains_all(digit, one)
    ).unwrap();
    let two = digits.iter().find(|digit|
        digit.len() == 5 && contains_n(digit, four, 2) && !is_equal(digit, three)
    ).unwrap();
    let five = digits.iter().find(|digit|
        digit.len() == 5 && !is_equal(digit, two) && !is_equal(digit, three)
    ).unwrap();

    let six = digits.iter().find(|digit|
        digit.len() == 6 && !contains_all(digit, one)
    ).unwrap();
    let zero = digits.iter().find(|digit|
        digit.len() == 6 && !contains_all(digit, four) && !is_equal(digit, six)
    ).unwrap();
    let nine = digits.iter().find(|digit|
        digit.len() == 6 && !is_equal(digit, six) && !is_equal(digit, zero)
    ).unwrap();

    mappings.push(Digit { example: zero, value: 0 });
    mappings.push(Digit { example: one, value: 1 });
    mappings.push(Digit { example: two, value: 2 });
    mappings.push(Digit { example: three, value: 3 });
    mappings.push(Digit { example: four, value: 4 });
    mappings.push(Digit { example: five, value: 5 });
    mappings.push(Digit { example: six, value: 6 });
    mappings.push(Digit { example: seven, value: 7 });
    mappings.push(Digit { example: eight, value: 8 });
    mappings.push(Digit { example: nine, value: 9 });

    output_digits.into_iter()
        .fold(String::from(""), |digits, digit|
            format!(
                "{}{}",
                digits,
                mappings.iter()
                    .find(|mapping|
                        is_equal(digit, mapping.example)
                    ).unwrap()
                    .value,
            )
        )
        .parse().unwrap()
}

pub fn solve(input: &str) -> usize {
    input.lines()
        .fold(0, |sum, line|
            sum + get_value(line)
        )
}
