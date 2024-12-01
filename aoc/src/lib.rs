use std::{env, fmt, fs, time};

pub use fxhash::{FxHashMap, FxHashSet};

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

    let path = env::args().nth(1).expect("no path specified");
    let input = fs::read_to_string(path).expect("file not found");

    run_part("one", one, &input);
    println!();
    run_part("two", two, &input);
}
