# :gift::christmas_tree: Advent of Code 2025 :christmas_tree::sparkles:

These are my solutions to this year's [Advent of Code](https://adventofcode.com/2025/).

Solutions make use of `cargo-aoc` code helper ([here](https://github.com/gobanos/cargo-aoc)).

## Solutions

All solutions linked below:
| Day | Title | 1 :star: | 2 :star: | Solution | Rating |
|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2025/day/1)  | Secret Entrance                 | 5.49µs | 27.0µs | [day01.rs](./src/day01.rs) | :sunglasses: |
| [02](https://adventofcode.com/2025/day/2)  | Gift Shop                       | 17.4ms | 34.0ms | [day02.rs](./src/day02.rs) | :smirk: |
| [03](https://adventofcode.com/2025/day/3)  | Lobby                           | 168µs  | 2.86ms | [day03.rs](./src/day03.rs) | :astonished: |
| [04](https://adventofcode.com/2025/day/4)  | Printing Department             | 3.45ms | 12.6ms | [day04.rs](./src/day04.rs) | :persevere: |
| [05](https://adventofcode.com/2025/day/5)  | Cafeteria                       | 298µs  | 8.43µs | [day05.rs](./src/day05.rs) | :scream: |
| [06](https://adventofcode.com/2025/day/6)  | Trash Compactor                 | 3.57µs | 72.8µs | [day06.rs](./src/day06.rs) | :no_mouth: |

## Notes
1. I'll revisit day 2 again as I think I can improve the algorithm.
2. It took a few optimisations to get day 3 part 2 to single digit ms.
3. Solutions should be run in both development and release profiles to see timing differences.
4. Day 4 part 2 was a tough one to run in parallel.
5. Day 5 initially required 4.9TB of RAM. Fortunately there was another data structure to the rescue.
6. Day 6 part 1 was pretty standard. Part 2 could have been easier, but I wanted a unified solution for both parts.