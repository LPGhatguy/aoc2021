static INPUT: &str = include_str!("../input.txt");

fn part1() {
    let bits = INPUT.lines().next().unwrap().len();
    let values: Vec<u16> = INPUT
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bits {
        let mut ones = 0;
        let mut zeroes = 0;

        for &val in &values {
            if val & (1 << i) == 0 {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }

        if ones > zeroes {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    println!("Part one: {}", gamma * epsilon);
}

fn solve(bits: usize, input: &[u16], ones: bool) -> u16 {
    let mut candidates = input.to_vec();

    loop {
        for i in (0..bits).rev() {
            if candidates.len() == 1 {
                return candidates[0];
            }

            let mut counter = 0i32;

            for &val in &candidates {
                if val & (1 << i) == 0 {
                    counter -= 1;
                } else {
                    counter += 1;
                }
            }

            if (counter >= 0) ^ ones {
                candidates.retain(|v| v & (1 << i) != 0);
            } else {
                candidates.retain(|v| v & (1 << i) == 0);
            }
        }
    }
}

fn part2() {
    let bits = INPUT.lines().next().unwrap().len();
    let values: Vec<u16> = INPUT
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect();

    let oxygen = solve(bits, &values, false);
    let co2 = solve(bits, &values, true);

    println!("{}", (oxygen as u32) * (co2 as u32));
}

fn main() {
    part1();
    part2();
}
