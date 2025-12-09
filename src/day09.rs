use crate::prelude::*;
use rayon::prelude::*;
use std::ops::Range;
use std::sync::{Arc, Mutex};

type Point = (usize, usize);
type Points = Vec<(usize, usize)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> (Points, Points) {
    let mut x_unique = HashSet::new();
    let mut y_unique = HashSet::new();

    let points: Vec<Point> = input
        .lines()
        .flat_map(|line| {
            let mut parts = line.split(',');
            let (x, y) = (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap());
            x_unique.insert(x);
            y_unique.insert(y);
            Some((x, y))
        })
        .collect();

    let mut x_map: Vec<usize> = x_unique.into_iter().collect();
    x_map.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let mut y_map: Vec<usize> = y_unique.into_iter().collect();
    y_map.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    
    let mut compressed = Vec::new();
    for (x, y) in points.iter() {
        let x_idx = x_map.iter().position(|&pos_x| pos_x == *x).unwrap();
        let y_idx = y_map.iter().position(|&pos_y| pos_y == *y).unwrap();
        compressed.push((x_idx, y_idx));
    }

    (points, compressed)
}

fn calc_area(point_a: &Point, point_b: &Point) -> usize {
    fn delta(a: usize, b: usize) -> usize {
        if a > b {
            return a - b + 1;
        } else {
            return b - a + 1;
        }
    }

    delta(point_a.0, point_b.0) * delta(point_a.1, point_b.1)
}

// Checks if any point is within the given rectangle
fn bounding_box(perimeter: &HashSet<Point>, points: &Points, point_a: &usize, point_b: &usize) -> bool {
    fn invalid_range(a: usize, b: usize) -> Range<usize> {
        if a > b {
            return (b + 1)..(a);
        } else {
            return (a + 1)..(b);
        }
    }

    let point1 = points[*point_a];
    let point2 = points[*point_b];

    let invalid_x = invalid_range(point1.0, point2.0);
    let invalid_y = invalid_range(point1.1, point2.1);

    // This works, but is slow. It turns out using HashSets is even slower.
    for point in perimeter {
        if invalid_x.contains(&point.0) && invalid_y.contains(&point.1) {
            return false;
        }
    }

    true
}

fn generate_perimeter(input: &Points) -> HashSet<Point> {
    let mut perimeter = HashSet::new();

    fn connect_points((x1, y1): Point, (x2, y2): Point) -> Points {
    let mut line = Vec::new();

    let (start_x, end_x, start_y, end_y) = if x1 <= x2 {
        (x1, x2, y1, y2)
    } else {
        (x2, x1, y2, y1)
    };

    if start_x == end_x {
        for y in min(start_y, end_y)..=max(start_y, end_y) {
            line.push((start_x, y));
        }
    } else {
        for x in start_x..=end_x {
            line.push((x, start_y));
        }
    }

    line
}

    for window in input.windows(2) {
        perimeter.extend(connect_points(window[0], window[1]));
    }
    perimeter.extend(connect_points(*input.last().unwrap(), input[0]));

    perimeter
}

#[aoc(day9, part1)]
pub fn solve_part1((points, _): &(Points, Points)) -> usize {
    let size = points.len();
    let mut largest = 0;

    (0..size)
        .flat_map(|point_a| (point_a + 1..size).map(move |point_b| (point_b, point_a)))
        .filter(|&(point_b, point_a)| point_b > point_a)
        .for_each(|(point_b, point_a)| {
            let area = calc_area(&points[point_a], &points[point_b]);
            if area > largest {
                largest = area;
            }
        });

    largest
}

#[aoc(day9, part2)]
// pub fn solve_part2((points, coods): &(Points, Coords)) -> usize {
//     let size = points.len();
//     let largest = Arc::new(Mutex::new(0));
//     let perimeter = generate_perimeter(points);

//     (0..size)
//         .into_par_iter()
//         .for_each(|point_a| {
//             (point_a + 1..size).into_par_iter().for_each(|point_b| {
//                 if bounding_box(&perimeter, points, &point_a, &point_b) {
//                     let area = calc_area(&points[point_a], &points[point_b]);
//                     let mut largest_inner = largest.lock().unwrap();
//                     if area > *largest_inner {
//                         *largest_inner = area;
//                     }
//                 }
//             });
//         });

//     *largest.lock().unwrap()
// }
// This version uses coordinate compression. Now runs 200x faster! It breaks the test though.
pub fn solve_part2((points, coords): &(Points, Points)) -> usize {
    let size = points.len();
    let largest = Arc::new(Mutex::new(0));
    let perimeter = generate_perimeter(&coords);

    (0..size)
        .into_par_iter()
        .for_each(|point_a| {
            (point_a + 1..size).into_par_iter().for_each(|point_b| {
                if bounding_box(&perimeter, &coords, &point_a, &point_b) {
                    let area = calc_area(&points[point_a], &points[point_b]);
                    let mut largest_inner = largest.lock().unwrap();
                    if area > *largest_inner {
                        *largest_inner = area;
                    }
                }
            });
        });

    *largest.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 50);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 24);
    }
}