const STEPS: usize = 100;

#[derive(Debug)]
struct Cell {
    energy: usize,
    has_flashed: bool,
}

pub fn solve_part_one(input: &str) -> usize {
    let mut cells = process(input);
    let mut flash_count = 0;

    for _step in 0..STEPS {
        for row in 0..cells.len() {
            for col in 0..cells[row].len() {
                increment_cell(&mut cells, row, col, &mut flash_count);
            }
        }

        cells.iter_mut()
            .for_each(|row| row.iter_mut()
                .for_each(|cell| {
                    if cell.energy > 9 { cell.energy = 0; }
                    cell.has_flashed = false;
                })
            );
    }

    flash_count
}

fn process(input: &str) -> Vec<Vec<Cell>> {
    input.lines()
        .map(|line| line.chars()
            .map(|energy| Cell {
                energy: energy.to_string().parse().unwrap(),
                has_flashed: false,
            })
            .collect()
        )
        .collect()
}

fn increment_cell(cells: &mut Vec<Vec<Cell>>, row: usize, col: usize, flash_count: &mut usize) {
    cells[row][col].energy += 1;
    let should_flash = !cells[row][col].has_flashed && cells[row][col].energy > 9;

    if should_flash {
        cells[row][col].has_flashed = true;
        *flash_count += 1;

        let n_row_min = if row > 0 { row - 1 } else { row };
        let n_row_max = if row < cells.len() - 1 { row + 2 } else { row + 1 };
        let n_col_min = if col > 0 { col - 1 } else { col };
        let n_col_max = if col < cells[row].len() - 1 { col + 2 } else { col + 1 };

        for n_row in n_row_min..n_row_max {
            for n_col in n_col_min..n_col_max {
                if !(n_row == row && n_col == col) {
                    increment_cell(cells, n_row, n_col, flash_count);
                }
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let mut cells = process(input);
    let mut flash_count = 0;
    let mut steps = 0;

    loop {
        steps += 1;
        for row in 0..cells.len() {
            for col in 0..cells[row].len() {
                increment_cell(&mut cells, row, col, &mut flash_count);
            }
        }

        let is_synchronised = cells.iter()
            .all(|row| row.iter()
                .all(|cell| cell.has_flashed)
            );

        if is_synchronised {
            return steps;
        }

        cells.iter_mut()
            .for_each(|row| row.iter_mut()
                .for_each(|cell| {
                    if cell.energy > 9 { cell.energy = 0; }
                    cell.has_flashed = false;
                })
            );
    }
}

fn print_cells(cells: &Vec<Vec<Cell>>) {
    for row in cells {
        for cell in row {
            print!("{}", cell.energy);
        }
        print!("\n");
    }
    print!("\n");
}
