#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(Dir, isize, usize)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_at(1);
            (
                match l.chars().next() {
                    Some('L') => Dir::Left,
                    Some('R') => Dir::Right,
                    _ => panic!("Invalid direction"),
                },
                r.parse::<isize>().unwrap() % 100,
                r.parse::<usize>().unwrap() / 100,
            )
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<(Dir, isize, usize)>) -> usize {
    let mut dial = 50;
    let mut count = 0;

    for (dir, steps, _) in input {
        match dir {
            Dir::Left => dial -= steps,
            Dir::Right => dial += steps,
        }

        if dial > 99 {
            dial -= 100;
        } else if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<(Dir, isize, usize)>) -> usize {
    let mut dial = 50;
    let mut count = 0;

    for (dir, steps, spins) in input {
        let mut add_count = false;
        let dial_start = dial;

        match dir {
            Dir::Left => dial -= steps,
            Dir::Right => dial += steps,
        }

        if dial > 99 {
            dial -= 100;
            add_count = true;
        } else if dial < 0 {
            dial += 100;
            if dial_start != 0 {
                add_count = true;
            }
        }

        if spins > &0 {
            count += spins;
        }

        if dial == 0 || add_count {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 6);
    }
}