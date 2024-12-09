use std::{env, fmt, fs, time};

pub use arrayvec::{self, ArrayString, ArrayVec};
pub use bitvec::{self, array::BitArray, slice::BitSlice, vec::BitVec};
pub use fxhash::{self, FxHashMap, FxHashSet};
pub use rayon::{self, prelude::*};
pub use regex::{self, Regex};
pub use slab::{self, Slab};
pub use smallvec::{self, SmallVec};
pub use vector_map::{self, set::VecSet, VecMap};

pub fn run_parts<OutputOne: fmt::Display, OutputTwo: fmt::Display>(
    one: impl FnOnce(&str) -> OutputOne,
    two: impl FnOnce(&str) -> OutputTwo,
) {
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
