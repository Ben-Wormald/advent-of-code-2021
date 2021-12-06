const DAYS_PART_ONE: usize = 80;
const DAYS: usize = 256;

fn iter(timers: &mut Vec<usize>) {
    let time_0 = timers[0];

    for time in 0..8 {
        timers[time] = timers[time + 1];
    }
    timers[6] += time_0;
    timers[8] = time_0;
}

pub fn solve(input: &str) -> usize {
    let mut timers: Vec<usize> = vec![0; 9];

    input
        .lines()
        .next().unwrap()
        .split(",")
        .for_each(|time|
            timers[time.parse::<usize>().unwrap()] += 1
        );

    for _day in 0..DAYS {
        iter(&mut timers);
    }

    timers.into_iter().sum()
}
