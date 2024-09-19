/*
** src/macros.rs
*/

#[macro_export]
macro_rules! head {
    ($x:tt $($xx:tt),*) => {
        $x
    };
}

#[macro_export]
macro_rules! tail {
    ($x:tt) => {
        $x
    };
    ($x:tt $($xx:tt),*) => {
        tail!($xx)
    };
}

#[macro_export]
macro_rules! count {
    () => {
        0
    };
    ($x:tt) => {
        1
    };
    ($x:tt $($xx:tt),*) => {
        count!($xx) + 1
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! puzzle_modules {
    ($($year:expr; $module:ident),+) => {
        use crate::{head, tail};
        use crate::types::Solution;

        pub struct PuzzleModules {}

        impl PuzzleModules {
            const START_YEAR: usize = head!($($year)*);
            const END_YEAR: usize = tail!($($year)*) + 1;

            fn puzzle_count(year: usize) -> usize {
                match year {
                    $(
                        $year => $module::PuzzleSet::count(),
                    ),*
                    _ => panic!("PuzzleSet::puzzle_count_for_year: invalid year: {}", year),
                }
            }

            pub fn dispatch(year: usize, day: usize) -> Box<dyn Fn(String) -> Solution> {
                match year {
                    $(
                        $year => $module::PuzzleSet::dispatch(day),
                    ),*
                    _ => panic!("PuzzleSet::dispatch: invalid year: {}", year),
                }
            }
        }
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! puzzle_set {
    ($($module:ident),+) => {
        use crate::count;
        use crate::types::Solution;

        pub struct PuzzleSet {}

        impl PuzzleSet {
            pub fn count() -> usize {
                count!($($module)*)
            }

            pub fn dispatch(day: usize) -> Box<dyn Fn(String) -> Solution> {
                let puzzles = [
                    $(
                        $module::solve,
                    ),*
                ];
                if day < puzzles.len() {
                    Box::new(puzzles[day])
                } else {
                    panic!("PuzzleSet::dispatch: invalid day: {}", day)
                }
            }
        }
    };
}
