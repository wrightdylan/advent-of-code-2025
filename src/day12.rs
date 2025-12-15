pub struct Region {
    size: (usize, usize),
    quants: [usize; 6],
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<Region>) {
    let mut parts = input.split("\n\n");

    (
        (0..6)
            .map(|_| {
                parts
                    .next()
                    .unwrap()
                    .lines()
                    .skip(1)
                    .flat_map(|line| line.chars())
                    .filter(|&ch| ch == '#')
                    .count()
            })
            .collect(),
        parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(": ").unwrap();
                
                let size: (usize, usize) = match first.split_once('x') {
                    Some((width_str, height_str)) => (
                        width_str.parse().unwrap_or_default(),
                        height_str.parse().unwrap_or_default()
                    ),
                    None => unreachable!(),
                };

                let mut quants = [0; 6];
                second
                    .split_whitespace()
                    .zip(&mut quants)
                    .for_each(|(count, idx)| {
                        *idx = count.parse::<usize>().unwrap();
                    });

                Region { size, quants }
            })
            .collect()
    )
}

#[aoc(day12, part1)]
pub fn solve_part1((shapes, regions): &(Vec<usize>, Vec<Region>)) -> usize {
    let mut sum = 0;

    for region in regions {
        if (region.size.0 / 3) * (region.size.1 / 3) >= region.quants.iter().sum() {
            let required_area = region.quants.iter().enumerate().map(|(idx, &count)| count * shapes[idx]).sum::<usize>();
            if required_area <= region.size.0 * region.size.1 {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 2);
    }
}