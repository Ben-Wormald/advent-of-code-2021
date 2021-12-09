fn process(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| line
            .chars()
            .map(|height| height.to_string().parse().unwrap())
            .collect()
        )
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let heights = process(input);

    heights.iter()
        .enumerate()
        .fold(0, |risk_level, (row_idx, row)|
            risk_level + row.iter()
                .enumerate()
                .fold(0, |row_risk_level, (col_idx, height)|
                    if is_local_minimum(&heights, row_idx, col_idx) {
                        row_risk_level + height + 1
                    } else {
                        row_risk_level
                    }
                )
        )
}

fn is_local_minimum(heights: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize) -> bool {
    let height = heights[row_idx][col_idx];
    let mut neighbours = vec!();

    if row_idx > 0 { neighbours.push(heights[row_idx - 1][col_idx]) }
    if row_idx < heights.len() - 1 { neighbours.push(heights[row_idx + 1][col_idx]) }
    if col_idx > 0 { neighbours.push(heights[row_idx][col_idx -1]) }
    if col_idx < heights[row_idx].len() - 1 { neighbours.push(heights[row_idx][col_idx + 1]) }

    neighbours.into_iter()
        .all(|neighbour| height < neighbour)
}

#[derive(Debug)]
struct Basin {
    size: usize,
    min_row_idx: usize,
    min_col_idx: usize,
}

pub fn solve(input: &str) -> usize {
    let heights = process(input);

    let mut basins: Vec<Basin> = vec!();

    heights.iter()
        .enumerate()
        .for_each(|(row_idx, row)| row.iter()
                .enumerate()
                .for_each(|(col_idx, height)| {
                    if *height == 9 { return; }

                    let (min_row_idx, min_col_idx) = descend(&heights, row_idx, col_idx);

                    let mut found = false;
                    basins.iter_mut()
                        .for_each(|basin|
                            if basin.min_row_idx == min_row_idx && basin.min_col_idx == min_col_idx {
                                basin.size += 1;
                                found = true;
                            }
                        );

                    if !found {
                        basins.push(Basin {
                            size: 1,
                            min_row_idx,
                            min_col_idx,
                        });
                    }
                })
        );

    basins.sort_by(|basin_a, basin_b| basin_b.size.cmp(&basin_a.size));

    basins.into_iter()
        .take(3)
        .fold(1, |product, basin| product * basin.size)
}

fn descend(heights: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize) -> (usize, usize) {
    if is_local_minimum(heights, row_idx, col_idx) {
        (row_idx, col_idx)
    } else {
        let (lower_row_idx, lower_col_idx) = min_neighbour(&heights, row_idx, col_idx);
        descend(heights, lower_row_idx, lower_col_idx)
    }
}

fn min_neighbour(heights: &Vec<Vec<usize>>, row_idx: usize, col_idx: usize) -> (usize, usize) {
    let mut neighbours = vec!();

    if row_idx > 0 { neighbours.push((row_idx - 1, col_idx)) }
    if row_idx < heights.len() - 1 { neighbours.push((row_idx + 1, col_idx)) }
    if col_idx > 0 { neighbours.push((row_idx, col_idx - 1)) }
    if col_idx < heights[row_idx].len() - 1 { neighbours.push((row_idx, col_idx + 1)) }

    neighbours.into_iter()
        .fold(
            (row_idx, col_idx),
            |(min_row_idx, min_col_idx), (neighbour_row_idx, neighbour_col_idx)|
                if heights[neighbour_row_idx][neighbour_col_idx] < heights[min_row_idx][min_col_idx] {
                    (neighbour_row_idx, neighbour_col_idx)
                } else {
                    (min_row_idx, min_col_idx)
                }
        )
}
