use crate::prelude::*;
use rayon::prelude::*;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Map {
    Floor,
    Paper,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid<Map> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut entity = Vec::new();

    for line in input.lines() {
        for ch in line.chars() {
            let map_item = match ch {
                '.' => Map::Floor,
                '@' => Map::Paper,
                _ => unreachable!(),
            };
            entity.push(map_item);
        }
    }

    Grid::new(width, height, entity)
}

#[allow(unused)]
fn draw_map(map: &Grid<Map>) {
    let char_map = HashMap::from([
        (Map::Floor, '.'),
        (Map::Paper, '@'),
    ]);

    map.draw_enum_map(&char_map);
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Grid<Map>) -> usize {
    (0..input.height).into_par_iter()
        .map(|row| {
            (0..input.width).into_iter()
                .filter(|&col| input[(col, row)] == Map::Paper && input.neighbours_cando_as(&(col, row), Map::Paper).len() < 4)
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Grid<Map>) -> usize {
    let mut map = input.clone();
    let mut removeables = true;
    let sum = Arc::new(AtomicUsize::new(0));
    let removal = Arc::new(Mutex::new(Vec::new()));

    while removeables {
        (0..map.height).into_par_iter().for_each(|row| {
            let mut removal_inner = Vec::new();
            (0..map.width).into_iter().for_each(|col| {
                if map[(col, row)] == Map::Paper && map.neighbours_cando_as(&(col, row), Map::Paper).len() < 4 {
                    sum.fetch_add(1, Ordering::Relaxed);
                    removal_inner.push((col, row));
                }
            });

            let mut removal_guard = removal.lock().unwrap();
            removal_guard.extend(removal_inner);
        });

        if !removal.lock().unwrap().is_empty() {
            map.place_at(&removal.lock().unwrap().clone(), Map::Floor);
            removal.lock().unwrap().clear();
        } else {
            removeables = false
        }
    }

    sum.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 13);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 43);
    }
}