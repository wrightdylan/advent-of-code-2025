use rayon::prelude::*;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;

    for line in input {
        let (mut first, mut second) = (line[0], line[1]);

        for num in &line[2..] {
            if second > first {
                first = second;
                second = *num;
            } else if num > &second {
                second = *num;
            }
        }

        sum += first * 10 + second;
    }

    sum
}

fn find_first_smallest(digits: &[usize]) -> Option<usize> {
    for (index, window) in digits.windows(2).enumerate() {
        if window[0] == 1 {
            return Some(index);
        }
        
        if window[0] < window[1] {
            return Some(index);
        }
    }

    None
}

fn shift_left(digits: &mut [usize], index: usize) {
    for idx in index..digits.len() - 1 {
        digits[idx] = digits[idx + 1];
    }
}

fn digits_to_number(digits: &[usize]) -> usize {
    digits.iter().rev().enumerate().fold(0, |acc, (i, &digit)| {
        acc + digit * 10usize.pow(i as u32)
    })
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<usize>>) -> usize {
    input.par_iter()
        .map(|line| {
            let mut digits: [usize; 12] = [0; 12];
            for (idx, &num) in line.iter().take(12).enumerate() {
                digits[idx] = num;
            }

            for num in &line[12..] {
                if let Some(index) = find_first_smallest(&digits) {
                    shift_left(&mut digits, index);
                    digits[11] = *num;
                } else if num > &digits[11] {
                    digits[11] = *num;
                }
            }

            digits_to_number(&digits)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 357);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 3121910778619);
    }
}