use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

fn coord(input: &str) -> (i32, i32) {
    let mut pieces = input.split(",");
    let x = pieces.next().unwrap().parse::<i32>().unwrap();
    let y = pieces.next().unwrap().parse::<i32>().unwrap();

    (x, y)
}

fn part1() {
    let lines = INPUT
        .lines()
        .map(|line| {
            let mut pieces = line.split(" -> ");

            let (ax, ay) = coord(pieces.next().unwrap());
            let (bx, by) = coord(pieces.next().unwrap());

            ((ax, ay), (bx, by))
        })
        .filter(|((ax, ay), (bx, by))| ax == bx || ay == by);

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for ((ax, ay), (bx, by)) in lines {
        let (mut x, mut y) = (ax, ay);
        let sx = (bx - ax).signum();
        let sy = (by - ay).signum();

        while (x, y) != (bx + sx, by + sy) {
            *points.entry((x, y)).or_default() += 1;
            x += sx;
            y += sy;
        }
    }

    let solution = points.values().filter(|x| **x >= 2).count();
    println!("Part one: {}", solution);
}

fn part2() {
    let lines = INPUT.lines().map(|line| {
        let mut pieces = line.split(" -> ");

        let (ax, ay) = coord(pieces.next().unwrap());
        let (bx, by) = coord(pieces.next().unwrap());

        ((ax, ay), (bx, by))
    });

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for ((ax, ay), (bx, by)) in lines {
        let (mut x, mut y) = (ax, ay);
        let sx = (bx - ax).signum();
        let sy = (by - ay).signum();

        while (x, y) != (bx + sx, by + sy) {
            *points.entry((x, y)).or_default() += 1;
            x += sx;
            y += sy;
        }
    }

    let solution = points.values().filter(|x| **x >= 2).count();
    println!("Part two: {}", solution);
}

fn main() {
    part1();
    part2();
}
