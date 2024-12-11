use std::{env, fmt, fs, time};

pub use arrayvec::{self, ArrayString, ArrayVec};
pub use bitvec::{self, array::BitArray, slice::BitSlice, vec::BitVec};
pub use fxhash::{self, FxHashMap, FxHashSet};
pub use rayon::{self, prelude::*};
pub use regex::{self, Regex};
pub use slab::{self, Slab};
pub use smallvec::{self, SmallVec};
pub use vector_map::{self, set::VecSet, VecMap};

pub mod grid;
pub use grid::*;

pub fn run_parts<OneOut, TwoOut>(one: impl FnOnce(&str) -> OneOut, two: impl FnOnce(&str) -> TwoOut)
where
    OneOut: fmt::Display,
    TwoOut: fmt::Display,
{
    fn run_part<Output: fmt::Display>(part: &str, func: impl FnOnce(&str) -> Output, input: &str) {
        let now = time::Instant::now();
        let out = func(input);
        let dur = now.elapsed();

        println!("Part {part}:\t{out}");
        println!("Elapsed:\t{dur:?}");
    }

    let mut args = env::args();
    let path = args.nth(1).expect("no path specified");
    let input = fs::read_to_string(path).expect("file not found");

    if let Some(path_two) = args.next() {
        let input_two = fs::read_to_string(path_two).expect("file not found");
        run_part("one", one, &input);
        run_part("two", two, &input_two);
    } else {
        run_part("one", one, &input);
        run_part("two", two, &input);
    }
}

pub fn run_parts_preproc<In, OneOut, TwoOut>(
    one: impl FnOnce(In) -> OneOut,
    two: impl FnOnce(In) -> TwoOut,
    preproc: impl Fn(&str) -> In,
) where
    OneOut: fmt::Display,
    TwoOut: fmt::Display,
{
    run_parts(|input| one(preproc(input)), |input| two(preproc(input)));
}
