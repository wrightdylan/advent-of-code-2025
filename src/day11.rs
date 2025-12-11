use crate::prelude::*;

type Devices = HashMap<usize, Vec<usize>>;
type Dict = HashMap<String, usize>;

// Using nodes as numeric IDs speeds up processing... a lot.
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> (Devices, Dict) {
    let mut dict = HashMap::new();

    input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            Some((line.get(0..3)?, idx))
        })
        .for_each(|(key, idx)| {
            dict.insert(key.to_string(), idx);
        });

    dict.insert("out".to_string(), dict.len());

    let devices = input
        .lines()
        .map(|line| {
            let (node, out) = line.split_once(": ").unwrap();
            (*dict.get(node).unwrap(), out.split_whitespace().map(|node| *dict.get(node).unwrap()).collect())
        })
        .collect();

    (devices, dict)
}

fn count_paths(current: &usize, target: &usize, devices: &Devices, cache: &mut Vec<usize>) -> usize {
    let cached = cache[*current];
    if cached != usize::MAX {
        return cached;
    }

    let mut result = 0;
    // It remains a mystery why the node check isn't returning 1 when it's
    // supposed to, and continues to pass a non-existent node to the next
    // iteration.
    if let Some(node_list) = devices.get(current) {
        for &node in node_list {
            if node == *target {
                return 1;
            } else {
                result += count_paths(&node, target, devices, cache);
            }
        }
    }

    cache[*current] = result;

    result
}

#[aoc(day11, part1)]
pub fn solve_part1((devices, dict): &(Devices, Dict)) -> usize {
    let you = dict.get("you").unwrap();
    let out = dict.get("out").unwrap();

    count_paths(you, out, &devices, &mut vec![usize::MAX; dict.len()])
}

// Rather than find all paths between 'svr' and 'out', and checking if 'dac' and
// 'fft' are in those paths, I chose to check 'svr' to 'dac' and 'svr' to 'fft'
// to see which is closer. As it turns out, the sequence is svr-fft-dac-out. Find
// all paths between each stage and multiply them.
#[aoc(day11, part2)]
pub fn solve_part2((devices, dict): &(Devices, Dict)) -> usize {
    let svr = dict.get("svr").unwrap();
    let fft = dict.get("fft").unwrap();
    let dac = dict.get("dac").unwrap();
    let out = dict.get("out").unwrap();

    let svr_fft = count_paths(svr, fft, &devices, &mut vec![usize::MAX; dict.len()]);
    let fft_dac = count_paths(fft, dac, &devices, &mut vec![usize::MAX; dict.len()]);
    let dac_out = count_paths(dac, out, &devices, &mut vec![usize::MAX; dict.len()]);

    svr_fft * fft_dac * dac_out
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 5);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 2);
    }
}