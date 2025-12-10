use binarray::BinaryArray;
use indicatif::{ProgressBar, ProgressStyle};
use crate::prelude::*;
use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

pub struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<usize>,
}

impl Machine {
    fn convert_buttons(&self) -> Vec<Vec<usize>> {
        self.buttons.iter().map(|button| button.to_indices()).collect()
    }

    fn fewest_presses_joltage(&self) -> usize {
        let buttons = self.convert_buttons();
        let opt = Optimize::new();
        let total = Int::fresh_const("total");

        let presses: Vec<Int> = (0..buttons.len())
            .map(|idx| Int::fresh_const(&format!("button_{idx}")))
            .collect();

        presses.iter().for_each(|button| opt.assert(&button.ge(0)));

        for (pos, &target) in self.joltage.iter().enumerate() {
            let mut terms = Vec::new();

            for (idx, button) in buttons.iter().enumerate() {
                if button.contains(&pos) {
                    terms.push(presses[idx].clone());
                }
            }
            let sum = Int::add(&terms.iter().collect::<Vec<&Int>>());
            opt.assert(&sum.eq(Int::from_u64(target as u64)));
        }

        opt.assert(&total.eq(Int::add(&presses)));
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => opt
                .get_model()
                .unwrap()
                .eval(&total, true)
                .and_then(|t| t.as_u64())
                .unwrap() as usize,
            _              => panic!("No solution found"),
        }
    }

    fn fewest_presses_lights(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((state, n)) = queue.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            if state == self.target {
                return n;
            }
            visited.insert(state.clone());
            for button in self.buttons.clone() {
                let mut next = state.clone();
                next ^= button;
                queue.push_back((next, n + 1));
            }
        }

        unreachable!()
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        let first = parts.remove(0);

        // Machine variables
        let mut target = 0_u16;
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();

        for (idx, ch) in first[1..first.len()-1].chars().enumerate() {
            if ch == '#' {
                target.set_bit(idx, true);
            }
        }

        for part in parts {
            // Split up each capsule
            let cap = part.chars().next().unwrap(); 
            let interior = &part[1..part.len()-1];
            let values: Vec<usize> = interior.split(',').map(|num| num.parse::<usize>().unwrap()).collect();
            match cap {
                '(' => {
                    let mut button = 0_u16;
                    for index in values {
                        button.set_bit(index, true);
                    }
                    buttons.push(button);
                },
                '{' => joltage = values,
                _   => panic!("Unexpected cap."),
            }
        }

        machines.push(Machine { target, buttons, joltage });
    }

    machines
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Machine>) -> usize {
    input
        .par_iter()
        .map(|machine| machine.fewest_presses_lights())
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Machine>) -> usize {
    let pb = ProgressBar::new(input.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})").unwrap());

    let total: usize = input
        .par_iter()
        .map(|machine| {
            let result = machine.fewest_presses_joltage();
            pb.inc(1);
            result
        })
        .sum();

    pb.finish();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 7);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 33);
    }
}