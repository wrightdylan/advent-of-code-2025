use rayon::prelude::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|range| range.split('-'))
        .filter_map(|mut ids| {
            Some((
                ids.next()?.parse().ok()?,
                ids.next()?.parse().ok()?
            ))
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<(usize, usize)>) -> usize {
    input
        .par_iter()
        .flat_map(|&(start, end)| {
            (start..=end).into_par_iter()
                .filter(|&n| {
                    let mag = n.ilog10() as usize + 1;
                    mag % 2 == 0
                })
                .filter(|&n| {
                    let num_str = n.to_string();
                    let (left, right) = num_str.split_at(num_str.len() / 2);
                    left == right
                })
                .map(|n| n)
        })
        .sum()
}

fn is_invalid(n: usize) -> bool {
    let num_str = n.to_string();
    let mag = num_str.len();

    for chunk_size in 1..=mag / 2 {
        if mag % chunk_size == 0 {
            let mut valid = true;
            for i in 0..mag-chunk_size {
                if num_str.chars().nth(i) != num_str.chars().nth(i + chunk_size) {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }
    }

    false
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<(usize, usize)>) -> usize {
    input
        .par_iter()
        .flat_map(|&(start, end)| {
            (start..=end).into_par_iter()
                .filter(|&n| {
                    is_invalid(n)
                })
                .map(|n| n)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 1227775554);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 4174379265);
    }
}