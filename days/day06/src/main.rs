static INPUT: &str = include_str!("../input.txt");

const GESTATION_TIMER: usize = 6;
const MATURITY_TIMER: usize = 2;

fn simulate<const N: usize>(ages: [u64; N]) -> [u64; N] {
    let mut new_ages = [0; N];

    let births = ages[0];
    new_ages[GESTATION_TIMER + MATURITY_TIMER] += births;
    new_ages[GESTATION_TIMER] += births;

    for i in 1..N {
        new_ages[i - 1] += ages[i];
    }

    new_ages
}

fn main() {
    let mut ages = [0u64; 9];

    INPUT
        .split(',')
        .map(|age| age.parse::<usize>().unwrap())
        .for_each(|age| {
            ages[age] += 1;
        });

    for _ in 0..80 {
        ages = simulate(ages);
    }

    let part_one: u64 = ages.iter().copied().sum();
    println!("Part one: {}", part_one);

    for _ in 0..(256 - 80) {
        ages = simulate(ages)
    }

    let part_two: u64 = ages.iter().copied().sum();
    println!("Part two: {}", part_two);
}
