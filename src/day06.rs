use ndarray::{Array2, Axis};
use std::collections::HashSet;

// pub struct Sheet {
//     numbers: Vec<Vec<usize>>,
//     operands: Vec<char>,
// }
pub struct Block {
    array: Array2<Option<char>>,
    oper: char,
}

impl Block {
    fn new(rows: usize, cols: usize, oper: char) -> Self {
        Self {
            array: Array2::from_elem((rows, cols), None),
            oper,
        }
    }

    fn calc(&self, axis: usize) -> usize {
        let mut sum = 0;

        for view in self.array.axis_iter(Axis(axis)) {
            let number = view
                .into_iter()
                .filter_map(|&option_ref| option_ref)
                .collect::<String>()
                .parse()
                .unwrap();

            if sum == 0 && self.oper == '*' {
                sum = number;
            } else {
                match self.oper {
                    '+' => sum += number,
                    '*' => sum *= number,
                    _   => unreachable!("There should not be any other operands.")
                }
            }
        }

        sum
    }
}

#[aoc_generator(day6)]
// pub fn input_generator(input: &str) -> Sheet {
//     let mut sheet = Vec::new();
//     let mut char_vec = Vec::new();

//     for line in input.lines() {
//         let mut line_vec = Vec::new();
//         for item in line.trim().split_whitespace() {
//             let ch = item.chars().next().unwrap();
//             if ch == '+' || ch == '*' {
//                 char_vec.push(ch);
//             } else {
//                 line_vec.push(item.parse().unwrap());
//             }
//         }

//         if !line_vec.is_empty() {
//             sheet.push(line_vec);
//         }
//     }

//     // let height = sheet.len();
//     // let width = sheet[0].iter().map(|s| s.len()).max().unwrap();

//     Sheet { numbers: sheet, operands: char_vec }
// }
pub fn input_generator(input: &str) -> Vec<Block> {
    let mut operands = Vec::new();
    let mut gaps = HashSet::new();
    let mut lines = input.lines();
    let mut blocks = Vec::new();

    let last_line = lines.next_back().unwrap_or("");

    // Array dimensions
    let mut widths = Vec::new();
    let height = lines.clone().count();

    // Process the operands and get the width of each block.
    // I was going to go with a square matrix, but this will
    // reduce the overhead.
    let mut width = 0;
    for (idx, ch) in last_line.chars().enumerate() {
        if ch == '+' || ch == '*' {
            operands.push(ch);
            if idx > 0 {
                gaps.insert(idx - 1);
                widths.push(width - 1);
                width = 0;
            }
        }

        width += 1;
    }
    widths.push(width);
    
    for idx in 0..widths.len() {
        blocks.push(Block::new(height, widths[idx], operands[idx]));
    }

    // Process the remaining lines
    for (row, line) in lines.enumerate() {
        let mut block_num = 0;
        let mut col_num = 0;
        for (idx, ch) in line.chars().enumerate() {
            if gaps.contains(&idx) {
                block_num += 1;
                col_num = 0;
            } else {
                if ch != ' ' {
                    blocks[block_num].array[(row, col_num)] = Some(ch);
                }
                col_num += 1;
            }
        }
    }

    blocks
}

#[aoc(day6, part1)]
// pub fn solve_part1(input: &Sheet) -> usize {
//     let mut sum = 0;

//     for (idx, op) in input.operands.iter().enumerate() {
//         let mut a = input.numbers[0][idx];
//         for num in 1..input.numbers.len() {
//             match op {
//                 '+' => a += input.numbers[num][idx],
//                 '*' => a *= input.numbers[num][idx],
//                 _   => panic!("Unreachable"),
//             }
//         }

//         sum += a;
//     }

//     sum
// }
pub fn solve_part1(input: &Vec<Block>) -> usize {
    let mut sum = 0;

    for block in input {
        sum += block.calc(0);
    }

    sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Block>) -> usize {
    let mut sum = 0;

    for block in input {
        sum += block.calc(1);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 4277556);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 3263827);
    }
}