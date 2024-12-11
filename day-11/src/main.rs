use aoc::FxHashMap;

fn main() {
    aoc::run_parts_preproc(one, two, |input| {
        input
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect()
    });
}

fn one(stones: Vec<u64>) -> u64 {
    let mut cache = FxHashMap::default();
    stones
        .into_iter()
        .map(|stone| split_stone(stone, 25, &mut cache))
        .sum()
}

fn two(stones: Vec<u64>) -> u64 {
    let mut cache = FxHashMap::default();
    stones
        .into_iter()
        .map(|stone| split_stone(stone, 75, &mut cache))
        .sum()
}

fn split_stone(stone: u64, blinks: u64, cache: &mut FxHashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(stones) = cache.get(&(stone, blinks)) {
        return *stones;
    }

    let stones = 'stones: {
        let blinks = blinks - 1;

        if stone == 0 {
            break 'stones split_stone(1, blinks, cache);
        }
        let exp = stone.ilog10();

        if exp % 2 == 1 {
            let mag = 10_u64.pow((exp + 1) / 2);
            split_stone(stone / mag, blinks, cache) + split_stone(stone % mag, blinks, cache)
        } else {
            split_stone(stone * 2024, blinks, cache)
        }
    };

    cache.insert((stone, blinks), stones);
    stones
}
