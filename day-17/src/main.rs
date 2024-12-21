use aoc::FxHashSet;
use std::fmt::Write;

fn main() {
    aoc::run_parts(one, two);
}

fn one(input: &str) -> String {
    let ((mut a, mut b, mut c), program) = parse_input(input);
    let ops = parse_program(&program);

    let mut out = String::new();

    let mut i = 0;
    while i < ops.len() {
        let st = (a, b, c);
        match ops[i] {
            Op::Adv(o) => a >>= resolve(st, o),
            Op::Bxl(o) => b ^= o,
            Op::Bst(o) => b = resolve(st, o) & 0b111,
            Op::Jnz(o) => {
                if a != 0 {
                    i = o;
                    continue;
                }
            }
            Op::Bxc => b ^= c,
            Op::Out(o) => {
                let o = resolve(st, o) & 0b111;
                if !out.is_empty() {
                    out.push(',');
                }
                out.write_fmt(format_args!("{o}")).unwrap();
            }
            Op::Bdv(o) => b = a >> resolve(st, o),
            Op::Cdv(o) => c = a >> resolve(st, o),
        }

        i += 1;
    }

    out
}

fn two(input: &str) -> u64 {
    let (_, program) = parse_input(input);
    let ops = parse_program(&program);

    let (jnz, cycle) = ops.split_last().unwrap();
    assert_eq!(*jnz, Op::Jnz(0));

    // assume adv shift constant
    let shift = ops
        .iter()
        .find_map(|op| match op {
            Op::Adv(Combo::Lit(shift)) => Some(shift),
            _ => None,
        })
        .unwrap();

    let mut candidates = FxHashSet::default();
    candidates.insert(0);

    for out in program.iter().rev().copied() {
        let mut new_candidates = FxHashSet::default();

        for candidate in candidates {
            for bottom in 0..1 << shift {
                let mut matches_out = false;

                // assume b and c depend solely on the current value of a
                let (mut a, mut b, mut c) = (candidate << shift | bottom, 0, 0);

                for op in cycle.iter().copied() {
                    let st = (a, b, c);
                    match op {
                        Op::Adv(_) => a >>= shift,
                        Op::Bxl(o) => b ^= o,
                        Op::Bst(o) => b = resolve(st, o) & 0b111,
                        Op::Jnz(_) => panic!("assume no jnz before end"),
                        Op::Bxc => b ^= c,
                        Op::Out(o) => {
                            let o = resolve(st, o) & 0b111;
                            matches_out = o == out;
                        }
                        Op::Bdv(o) => b = a >> resolve(st, o),
                        Op::Cdv(o) => c = a >> resolve(st, o),
                    }
                }

                if matches_out {
                    new_candidates.insert(candidate << shift | bottom);
                }
            }
        }

        candidates = new_candidates;
    }

    candidates.iter().copied().min().unwrap()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Op {
    Adv(Combo),
    Bxl(u64),
    Bst(Combo),
    Jnz(usize),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Combo {
    Lit(u64),
    A,
    B,
    C,
}

fn combo(operand: u64) -> Combo {
    match operand {
        0..=3 => Combo::Lit(operand),
        4 => Combo::A,
        5 => Combo::B,
        6 => Combo::C,
        _ => panic!(),
    }
}

fn resolve((a, b, c): (u64, u64, u64), operand: Combo) -> u64 {
    match operand {
        Combo::Lit(operand) => operand,
        Combo::A => a,
        Combo::B => b,
        Combo::C => c,
    }
}

fn parse_input(input: &str) -> ((u64, u64, u64), Vec<u64>) {
    use aoc::parse::*;

    let (a, input) = seq!("Register A: ", uint, "\n")(input);
    let (b, input) = seq!("Register B: ", uint, "\n")(input);
    let (c, input) = seq!("Register C: ", uint, "\n")(input);

    let (_, input) = seq!("\nProgram: ")(input);
    let ops = sep_by(",", uint)(input).map(|(op, _)| op).collect();

    ((a, b, c), ops)
}

fn parse_program(program: &[u64]) -> Vec<Op> {
    program
        .chunks_exact(2)
        .map(|op| {
            let (opcode, o) = (op[0], op[1]);
            match opcode {
                0 => Op::Adv(combo(o)),
                1 => Op::Bxl(o),
                2 => Op::Bst(combo(o)),
                3 => Op::Jnz(o as usize / 2),
                4 => Op::Bxc,
                5 => Op::Out(combo(o)),
                6 => Op::Bdv(combo(o)),
                7 => Op::Cdv(combo(o)),
                _ => panic!(),
            }
        })
        .collect()
}
