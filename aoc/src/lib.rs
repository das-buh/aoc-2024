use std::{
    env,
    fmt::{Display, Formatter, Result},
    fs, time,
};

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

pub mod parse;

pub fn run_parts<OneAns, TwoAns>(one: impl FnOnce(&str) -> OneAns, two: impl FnOnce(&str) -> TwoAns)
where
    DisplayAnswer<OneAns>: Display,
    DisplayAnswer<TwoAns>: Display,
{
    fn run_part<Ans>(part: &str, func: impl FnOnce(&str) -> Ans, input: &str)
    where
        DisplayAnswer<Ans>: Display,
    {
        let now = time::Instant::now();
        let out = func(input);
        let dur = now.elapsed();

        println!("Part {part}:\t{}", DisplayAnswer(out));
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

pub struct DisplayAnswer<T>(T);

macro_rules! impl_display {
    ($( $ty:ty ),* $(,)?) => {
        $(
            impl Display for DisplayAnswer<$ty> {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_fmt(format_args!("{}", self.0))
                }
            }
        )*
    };
}

macro_rules! impl_display_tuple {
    () => {};
    (recurse: $_:ident, $( $rest:ident, )*) => {
        impl_display_tuple! { $( $rest, )* }
    };
    ($( $elems:ident ),* $(,)?) => {
        impl<T0: Display, $( $elems : Display ),*> Display for DisplayAnswer<(T0, $( $elems ),*)> {
            #[allow(non_snake_case)]
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let (first, $( $elems ),*) = &self.0;
                f.write_fmt(format_args!("{first}"))?;
                $( f.write_fmt(format_args!(",{}", $elems))?; )*
                Ok(())
            }
        }

        impl_display_tuple! { recurse: $( $elems, )* }
    };
}

macro_rules! impl_display_iter {
    () => {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let mut iter = self.0.iter();
            if let Some(elem) = iter.next() {
                f.write_fmt(format_args!("{elem}"))?;
            }
            for elem in iter {
                f.write_fmt(format_args!(",{elem}"))?;
            }
            Ok(())
        }
    };
}

impl Display for DisplayAnswer<()> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("ok")
    }
}

impl_display! { u8, u16, u32, u64, i8, i16, i32, i64, &str, String }

impl_display_tuple! { T1, T2, T3, T4, T5, T6, T7 }

impl<T: Display> Display for DisplayAnswer<&[T]> {
    impl_display_iter!();
}

impl<T: Display, const N: usize> Display for DisplayAnswer<[T; N]> {
    impl_display_iter!();
}

impl<T: Display> Display for DisplayAnswer<Vec<T>> {
    impl_display_iter!();
}
