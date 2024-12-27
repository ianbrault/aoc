/*
** src/macros.rs
*/

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! puzzle_modules {
    ($($year:expr => $module:ident),+) => {
        $(mod $module;)*

        pub struct PuzzleModules {}

        impl PuzzleModules {
            fn years() -> Vec<usize> {
                vec![$($year,)*]
            }

            fn puzzle_count(year: usize) -> usize {
                match year {
                    $(
                        $year => $module::PuzzleSet::count(),
                    )*
                    _ => panic!("PuzzleSet::puzzle_count_for_year: invalid year: {}", year),
                }
            }

            pub fn dispatch(year: usize, day: usize) -> fn(String) -> Solution {
                match year {
                    $(
                        $year => $module::PuzzleSet::dispatch(day),
                    )*
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
        use crate::puzzles::Solution;

        $(mod $module;)*

        pub struct PuzzleSet {}

        impl PuzzleSet {
            pub fn count() -> usize {
                [
                    $(
                        $module::solve,
                    )*
                ].len()
            }

            pub fn dispatch(day: usize) -> fn(String) -> Solution {
                let puzzles = [
                    $(
                        $module::solve,
                    )*
                ];
                if day < puzzles.len() {
                    puzzles[day]
                } else {
                    panic!("PuzzleSet::dispatch: invalid day: {}", day)
                }
            }
        }
    };
}
