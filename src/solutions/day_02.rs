enum Direction {
    Forward,
    Up,
    Down,
}

struct Command {
    direction: Direction,
    units: usize,
}

struct Position {
    horizontal: usize,
    depth: usize,
}

struct PositionWithAim {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

fn process(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            Command {
                direction: match parts[0] {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!("oh dear!"),
                },
                units: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let commands = process(input);

    let mut position = Position { horizontal: 0, depth: 0 };

    commands
        .into_iter()
        .for_each(|command| {
            match command.direction {
                Direction::Forward => position.horizontal += command.units,
                Direction::Up => position.depth -= command.units,
                Direction::Down => position.depth += command.units,
            }
        });

    position.horizontal * position.depth
}

pub fn solve(input: &str) -> usize {
    let commands = process(input);

    let mut position = PositionWithAim {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    commands
        .into_iter()
        .for_each(|command| {
            match command.direction {
                Direction::Forward => {
                    position.horizontal += command.units;
                    position.depth += position.aim * command.units;
                },
                Direction::Up => position.aim -= command.units,
                Direction::Down => position.aim += command.units,
            }
        });

    position.horizontal * position.depth
}
