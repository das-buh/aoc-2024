use aoc::FxHashMap;
use std::{
    iter::{once, repeat_n, Chain, Once, RepeatN},
    mem,
};

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> u64 {
    sum_complexities(input, |code| {
        let numpad = expand_seq(code.iter().copied(), num_pos, is_valid_num).flatten();
        let radio = expand_seq(numpad, dir_pos, is_valid_dir).flatten();
        let cold = expand_seq(radio, dir_pos, is_valid_dir).flatten();

        cold.count() as u64
    })
}

fn two(input: &str) -> u64 {
    sum_complexities(input, |code| {
        let code = Sequences::from_iter([(code.to_vec(), 1)]);
        let mut seqs = expand_seq_set(code, num_pos, is_valid_num);

        for _ in 0..25 {
            seqs = expand_seq_set(seqs, dir_pos, is_valid_dir);
        }

        seqs.into_iter()
            .map(|(seq, count)| seq.len() as u64 * count)
            .sum()
    })
}

fn sum_complexities(codes: &str, mut shortest_len: impl FnMut(&[u8]) -> u64) -> u64 {
    codes
        .lines()
        .map(|code| {
            let shortest_len = shortest_len(code.as_bytes());
            let num_part = code[..3].parse::<u64>().unwrap();
            shortest_len * num_part
        })
        .sum()
}

type Expand = Chain<Chain<RepeatN<u8>, RepeatN<u8>>, Once<u8>>;

fn expand_seq(
    seq: impl Iterator<Item = u8>,
    lookup: fn(u8) -> (i64, i64),
    is_valid: fn((i64, i64)) -> bool,
) -> impl Iterator<Item = Expand> {
    let mut prev = lookup(b'A');

    seq.map(move |curr| {
        let curr = lookup(curr);
        let (di, dj) = (curr.0 - prev.0, curr.1 - prev.1);

        let i = repeat_n(if di > 0 { b'v' } else { b'^' }, di.unsigned_abs() as usize);
        let j = repeat_n(if dj > 0 { b'>' } else { b'<' }, dj.unsigned_abs() as usize);

        let expand = if (dj < 0 || !is_valid((curr.0, prev.1))) && is_valid((prev.0, curr.1)) {
            j.chain(i)
        } else {
            i.chain(j)
        };

        prev = curr;
        expand.chain(once(b'A'))
    })
}

type Sequences = FxHashMap<Vec<u8>, u64>;

fn expand_seq_set(
    seqs: Sequences,
    lookup: fn(u8) -> (i64, i64),
    is_valid: fn((i64, i64)) -> bool,
) -> Sequences {
    let mut out = Sequences::default();
    let mut buf = Vec::new();

    for (seq, count) in seqs {
        for expand in expand_seq(seq.into_iter(), lookup, is_valid) {
            buf.extend(expand);
            *out.entry(mem::take(&mut buf)).or_insert(0) += count;
        }
    }

    out
}

fn num_pos(num: u8) -> (i64, i64) {
    match num {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!(),
    }
}

fn is_valid_num((i, j): (i64, i64)) -> bool {
    (i, j) != (3, 0)
}

fn dir_pos(dir: u8) -> (i64, i64) {
    match dir {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!(),
    }
}

fn is_valid_dir((i, j): (i64, i64)) -> bool {
    (i, j) != (0, 0)
}
