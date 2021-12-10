pub fn solve_part_one(input: &str) -> usize {
    input.lines()
        .fold(0, |sum, line| {
            let symbols = line.chars().collect();
            sum + get_corrupted_score(symbols)
        })
}

fn get_corrupted_score(symbols: Vec<char>) -> usize {
    let mut stack: Vec<char> = vec!();
    let mut is_incomplete = false;
    let mut score = 0;

    symbols.into_iter()
        .for_each(|symbol| {
            if score != 0 || is_incomplete { return; }

            if is_open_symbol(symbol) {
                stack.push(symbol);
            } else {
                match stack.pop() {
                    Some(open_symbol) => {
                        if !is_matching_close(open_symbol, symbol) {
                            score = get_symbol_score(symbol);
                        }
                    },
                    None => is_incomplete = true,
                }
            }
        });

    score
}

fn is_open_symbol(symbol: char) -> bool {
    match symbol {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn is_matching_close(open_symbol: char, close_symbol: char) -> bool {
    match open_symbol {
        '(' => close_symbol == ')',
        '[' => close_symbol == ']',
        '{' => close_symbol == '}',
        '<' => close_symbol == '>',
        _ => false,
    }
}

fn get_symbol_score(symbol: char) -> usize {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn solve(input: &str) -> usize {
    let mut scores: Vec<usize> = input.lines()
        .map(|line| {
            let symbols = line.chars().collect();
            get_incomplete_score(symbols)
        })
        .filter_map(|score| score)
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn get_incomplete_score(symbols: Vec<char>) -> Option<usize> {
    let mut stack: Vec<char> = vec!();
    let mut is_corrupt = false;

    symbols.into_iter()
        .for_each(|symbol| {
            if is_corrupt { return; }

            if is_open_symbol(symbol) {
                stack.push(symbol);
            } else {
                match stack.pop() {
                    Some(open_symbol) => {
                        if !is_matching_close(open_symbol, symbol) {
                            is_corrupt = true;
                        }
                    },
                    None => (),
                }
            }
        });

    if is_corrupt { return None; }

    Some(
        stack.into_iter()
            .rev()
            .fold(0, |score, symbol|
                score * 5 + get_incomplete_symbol_score(symbol)
            )
    )
}

fn get_incomplete_symbol_score(symbol: char) -> usize {
    match symbol {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
