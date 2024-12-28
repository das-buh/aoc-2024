use aoc::FxHashMap;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let mut price = line.parse().unwrap();

        for _ in 0..2000 {
            price = next(price);
        }

        sum += price;
    }

    sum
}

// this kind of sucks
fn two(input: &str) -> u64 {
    let mut bananas = FxHashMap::<[i8; 4], (u64, usize)>::default();

    for (buyer, line) in input.lines().enumerate() {
        let mut price = line.parse().unwrap();
        let mut changes = [0; 4];

        for i in 0..2000 {
            let new = next(price);
            changes.rotate_left(1);
            changes[3] = (new % 10) as i8 - (price % 10) as i8;

            price = new;

            if i < 3 {
                continue;
            }

            let (count, last_buyer) = bananas.entry(changes).or_insert((0, usize::MAX));
            if *last_buyer == buyer {
                continue;
            }

            *count += price % 10;
            *last_buyer = buyer;
        }
    }

    bananas
        .into_iter()
        .map(|(_, (count, _))| count)
        .max()
        .unwrap()
}

fn next(n: u64) -> u64 {
    const PRUNE: u64 = (1 << 24) - 1;
    let n = (n << 6 ^ n) & PRUNE;
    let n = (n >> 5 ^ n) & PRUNE;
    (n << 11 ^ n) & PRUNE
}
