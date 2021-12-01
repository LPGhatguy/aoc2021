use itertools::Itertools;

static INPUT: &str = include_str!("../input.txt");

fn part1() {
    let count = INPUT
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("Part one: {}", count);
}

fn part2() {
    let count = INPUT
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("Part two: {}", count);
}

fn main() {
    part1();
    part2();
}
