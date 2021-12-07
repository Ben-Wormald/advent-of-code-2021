pub fn solve_part_one(input: &str) -> isize {
    let mut distances: Vec<isize> = input
        .lines()
        .next().unwrap()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();

    distances.sort();

    let median = distances[distances.len() / 2];

    distances.into_iter()
        .fold(0, |sum, d| sum + (d - median).abs())
}

pub fn solve(input: &str) -> isize {
    let mut distances: Vec<isize> = input
        .lines()
        .next().unwrap()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();

    distances.sort();

    let mut min_fuel: Option<isize> = None;

    for d in distances[0]..distances[distances.len() - 1] {
        let fuel = distances.iter()
            .fold(0, |sum, start| {
                let travel = (start - d).abs();
                sum + travel * (travel + 1) / 2
            });

        match min_fuel {
            Some(min_f) => if fuel < min_f {
                min_fuel = Some(fuel);
            },
            None => {
                min_fuel = Some(fuel);
            },
        }
    }

    min_fuel.unwrap()
}
