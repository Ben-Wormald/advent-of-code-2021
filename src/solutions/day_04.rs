#[derive(Clone, Debug)]
struct Number {
    value: usize,
    is_marked: bool,
}

type Board = Vec<Vec<Number>>;

struct BoardWithState {
    board: Board,
    has_won: bool,
}

fn check_board(board: &Board) -> bool {
    for row in board.iter() {
        if row.iter().all(|number| number.is_marked) { return true }
    }
    for column in 0..4 {
        if board.iter().all(|row| row[column].is_marked) { return true }
    }
    false
}

fn get_score(board: &Board) -> usize {
    board.iter()
        .fold(0, |board_sum, row|
            board_sum + row.iter()
                .fold(0, |row_sum, number|
                    if number.is_marked {
                        row_sum
                    } else {
                        row_sum + number.value
                    }
                )
        )
}

fn process(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut lines: std::str::Lines = input.lines();

    let draws: Vec<usize> = lines
        .next().unwrap()
        .split(",")
        .map(|draw| draw.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec!();
    let mut board: Board = vec!();

    lines
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let row: Vec<Number> = line
                .split_whitespace()
                .map(|number| Number {
                    value: number.parse().unwrap(),
                    is_marked: false,
                })
                .collect();

            board.push(row);

            if board.len() == 5 {
                boards.push(board.clone());
                board = vec!();
            }
        });

    (draws, boards)
}

pub fn solve_part_one(input: &str) -> usize {
    let (draws, mut boards) = process(input);

    for draw in draws.into_iter() {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for number in row.iter_mut() {
                    if number.value == draw { number.is_marked = true };
                }
            }

            if check_board(board) { return draw * get_score(board); }
        }
    }

    0
}

pub fn solve(input: &str) -> usize {
    let (draws, boards) = process(input);

    let mut boards: Vec<BoardWithState> = boards.into_iter()
        .map(|board| BoardWithState {
            board,
            has_won: false,
        })
        .collect();

    let mut boards_won = 0;
    let board_count = boards.len();

    for draw in draws.into_iter() {
        for board_with_state in boards.iter_mut() {
            for row in board_with_state.board.iter_mut() {
                for number in row.iter_mut() {
                    if number.value == draw { number.is_marked = true };
                }
            }

            if !board_with_state.has_won && check_board(&board_with_state.board) {
                board_with_state.has_won = true;
                boards_won += 1;

                if boards_won == board_count {
                    return draw * get_score(&board_with_state.board)
                }
            }
        }
    }

    0
}
