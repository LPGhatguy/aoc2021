static INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Input {
    calls: Vec<u8>,
    boards: Vec<[[u8; 5]; 5]>,
}

impl Input {
    fn read(input: &str) -> Self {
        let mut lines = input.lines();

        let calls = lines
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u8>().unwrap())
            .collect();

        let lines: Vec<_> = lines.filter(|line| !line.is_empty()).collect();

        let mut boards = Vec::new();
        for rows in lines.chunks_exact(5) {
            let mut board = [[0; 5]; 5];

            for (x, row) in rows.iter().enumerate() {
                row.split_whitespace()
                    .take(5)
                    .enumerate()
                    .for_each(|(y, value)| {
                        board[x][y] = value.parse().unwrap();
                    });
            }

            boards.push(board);
        }

        Self { calls, boards }
    }
}

fn is_solved<const N: usize>(occupancy: [[bool; N]; N]) -> bool {
    occupancy.iter().any(|row| row.iter().all(|x| *x))
        || (0..N).any(|y| occupancy.iter().map(|row| row[y]).all(|x| x))
}

#[test]
fn solving() {
    assert_eq!(is_solved([[false, false], [false, false]]), false);
    assert_eq!(is_solved([[true, false], [false, true]]), false);
    assert_eq!(is_solved([[false, true], [true, false]]), false);

    assert_eq!(is_solved([[true, true], [false, false]]), true);
    assert_eq!(is_solved([[true, false], [true, false]]), true);
    assert_eq!(is_solved([[true, true], [true, true]]), true);
}

fn part1() {
    let input = Input::read(INPUT);

    let mut occupancy: Vec<_> = (0..input.boards.len()).map(|_| [[false; 5]; 5]).collect();

    for call in input.calls {
        for (board, occupancy) in input.boards.iter().zip(occupancy.iter_mut()) {
            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y] == call {
                        occupancy[x][y] = true;
                    }
                }
            }

            if is_solved(*occupancy) {
                let mut sum = 0;

                for x in 0..5 {
                    for y in 0..5 {
                        if !occupancy[x][y] {
                            sum += board[x][y] as u32;
                        }
                    }
                }

                println!("Day 1: {}", sum * call as u32);
                return;
            }
        }
    }
}

fn part2() {
    let input = Input::read(INPUT);

    let mut occupancy: Vec<_> = (0..input.boards.len()).map(|_| [[false; 5]; 5]).collect();
    let mut solved = vec![false; input.boards.len()];
    let mut num_solved = 0;

    for call in input.calls {
        for ((board, occupancy), solved) in input
            .boards
            .iter()
            .zip(occupancy.iter_mut())
            .zip(solved.iter_mut())
        {
            if *solved {
                continue;
            }

            for x in 0..5 {
                for y in 0..5 {
                    if board[x][y] == call {
                        occupancy[x][y] = true;
                    }
                }
            }

            if is_solved(*occupancy) {
                *solved = true;
                num_solved += 1;

                if num_solved == input.boards.len() {
                    let mut sum = 0;

                    for x in 0..5 {
                        for y in 0..5 {
                            if !occupancy[x][y] {
                                sum += board[x][y] as u32;
                            }
                        }
                    }

                    println!("Day 2: {}", sum * call as u32);
                    return;
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
