use crate::prelude::*;

type Distances = Vec<(u32, (usize, usize))>;
type Points = Vec<Vec<f32>>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Points, Distances) {
    let points: Vec<Vec<f32>> = input
        .lines()
        .map(|line| line
            .split(',')
            .map(|part| part
                .parse()
                .unwrap()
            ).collect()
        ).collect();

    fn calc_eucl_dist(points: &Vec<Vec<f32>>, idx1: usize, idx2: usize) -> f32 {
        (
            (points[idx1][0] - points[idx2][0]).powi(2) +
            (points[idx1][1] - points[idx2][1]).powi(2) +
            (points[idx1][2] - points[idx2][2]).powi(2)
        )
        .sqrt()
    }

    fn normalise_coords(col: usize, row: usize) -> (usize, usize) {
        if col < row {
            (col, row)
        } else {
            (row, col)
        }
    }

    let size = input.lines().clone().count();
    let mut distance_set = HashSet::new();
    (0..size)
        .flat_map(|col| (col + 1..size).map(move |row| (row, col)))
        .filter(|&(row, col)| row > col)
        .for_each(|(row, col)| {
            let distance = calc_eucl_dist(&points, col, row);
            if distance != 0.0 {
                distance_set.insert((distance.to_bits(), normalise_coords(col, row)));
            }
        });

    let mut distances: Vec<(u32, (usize, usize))> = distance_set.into_iter().collect();
    distances.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    (points, distances)
}

enum GroupErr {
    MergeGroups(usize, usize),
    SameGroup,
}

trait GroupOperations {
    fn check(&mut self, coords: &(usize, usize)) -> Result<Vec<(usize, usize)>, GroupErr>;
    fn merge(&mut self, idx1: usize, idx2: usize);
    fn singles(&self, points: &Points) -> Vec<usize>;
}

impl GroupOperations for Vec<HashSet<usize>> {
    fn check(&mut self, (a, b): &(usize, usize)) -> Result<Vec<(usize, usize)>, GroupErr> {
        let mut result = Vec::new();

        for (idx, group) in self.iter().enumerate() {
            if group.contains(a) && group.contains(b) {
                return Err(GroupErr::SameGroup);
            } else if group.contains(a) {
                result.push((idx, *b));
            } else if group.contains(b) {
                result.push((idx, *a));
            }
        }

        if result.len() == 2 && result[0].0 != result[1].0 {
            return Err(GroupErr::MergeGroups(result[0].0, result[1].0));
        }

        if result.is_empty() {
            result.push((self.len(), *a));
            result.push((self.len(), *b));
        }

        Ok(result)
    }

    fn merge(&mut self, idx1: usize, idx2: usize) {
        let next_set = self.remove(idx2);
        let current_set = &mut self[idx1];

        for item in next_set {
            current_set.insert(item);
        }
    }

    fn singles(&self, points: &Points) -> Vec<usize> {
        let mut singles = Vec::new();

        for idx in 0..points.len() {
            let mut contained = false;
            for group in self {
                if group.contains(&idx) {
                    contained = true;
                }
            }

            if !contained {
                singles.push(idx);
            }
        }

        singles
    }
}

#[aoc(day8, part1)]
pub fn solve_part1((_, distances): &(Points, Distances)) -> usize {
    let mut groups = Vec::new();
    let mut connections = 0;
    let mut idx = 0;

    while connections < 1000 {
        let (_, coords) = distances[idx];
        
        match groups.check(&coords) {
            Ok(result) => {
                if result.len() == 1 {
                    let (group, entry) = (result[0].0, result[0].1);
                    groups[group].insert(entry);
                    connections += 1;
                } else if result.len() == 2 {
                    groups.push(hashset!(coords.0, coords.1));
                    connections += 1;
                }
            },
            Err(GroupErr::MergeGroups(a, b)) => {
                groups.merge(a, b);
                connections += 1;
            },
            Err(GroupErr::SameGroup) => connections += 1,
        }

        idx += 1;
    }

    let mut lengths = Vec::new();
    for group in groups {
        lengths.push(group.len());
    }
    lengths.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

    lengths.iter().take(3).fold(1, |acc, &x| acc * x)
}

// Passes test, but run = 6171742720 is too low
#[aoc(day8, part2)]
pub fn solve_part2((points, distances): &(Points, Distances)) -> usize {
    let mut groups = Vec::new();
    let mut box_a = 0;
    let mut box_b = 0;
    let mut idx = 0;
    let mut mark = true;

    // for idx in 0..100 {
    loop {
        if groups.len() == 1 && !mark && groups.singles(points).is_empty() {
            break;
        }
        let (_, coords) = distances[idx];
        
        match groups.check(&coords) {
            Ok(result) => {
                if result.len() == 1 {
                    let (group, entry) = (result[0].0, result[0].1);
                    groups[group].insert(entry);
                } else if result.len() == 2 {
                    groups.push(hashset!(coords.0, coords.1));
                }
                box_a = coords.0;
                box_b = coords.1;
            },
            Err(GroupErr::MergeGroups(a, b)) => {
                groups.merge(a, b);
                box_a = a;
                box_b = b;
            },
            Err(GroupErr::SameGroup) => {},
        }

        if groups.len() > 1 {
            mark = false;
        }
        
        idx += 1;
    }

    println!("{:?}, {:?}", points[box_a], points[box_b]);
    println!("{:?}", groups.singles(points));
    println!("Groups: {}", groups.len());

    (points[box_a][0] * points[box_b][0]) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    // #[test]
    // fn part1_test() {
    //     assert_eq!(solve_part1(&input_generator(TEST)), 40);
    // }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 25272);
    }
}