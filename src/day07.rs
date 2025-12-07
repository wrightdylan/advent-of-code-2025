use crate::prelude::*;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .enumerate()
        .filter(|(row, _)| row % 2 == 0)
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, ch)| if ch != '.' { Some(col) } else { None })
                .collect()
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let mut beams: HashSet<usize> = hashset!(input[0][0]);
    let mut splits = 0;

    for line in input.iter().skip(1) {
        for pos in line {
            if beams.contains(pos) {
                beams.remove(pos);
                beams.insert(pos - 1);
                beams.insert(pos + 1);
                splits += 1;
            }
        }
    }
    
    splits
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    fn _split_time(splitters: &Vec<Vec<usize>>, index: usize, beam: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if let Some(&result) = cache.get(&(index, beam)) {
                return result;
            }

        if let Some(line) = splitters.get(index) {
            let result;
            if line.contains(&beam) {
                let result1 = _split_time(splitters, index + 1, beam - 1, cache);
                let result2 = _split_time(splitters, index + 1, beam + 1, cache);
                result = result1 + result2;
            } else {
                result = _split_time(splitters, index + 1, beam, cache);
            }

            cache.insert((index, beam), result);
            return result;
        }

        1
    }

    // Adding cache should solve my runtime problem as multiple beams converge
    // and diverge from the same point
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    _split_time(input, 1, input[0][0], &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 21);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 40);
    }
}