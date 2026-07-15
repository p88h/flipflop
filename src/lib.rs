pub struct Solution {
    pub year: i32,
    pub puzzle: i32,
    pub part1: fn(&str) -> String,
    pub part2: fn(&str) -> String,
    pub part3: fn(&str) -> String,
}

macro_rules! library {
    ($($year:tt { $($puzzle:tt),* }),*) => {
        $(
            pub mod $year {
                $(pub mod $puzzle;)*
            }
            pub fn $year() -> Vec<Solution> {
                vec![$({
                    use $year::$puzzle::*;
                    Solution {
                        year: stringify!($year)[1..].parse::<i32>().unwrap(),
                        puzzle: stringify!($puzzle)[1..].parse::<i32>().unwrap(),
                        part1: |notes: &str| part1(notes).to_string(),
                        part2: |notes: &str| part2(notes).to_string(),
                        part3: |notes: &str| part3(notes).to_string(),
                    }
                },)*]
            }
        )*

        pub fn solutions() -> Vec<Solution> {
            let mut sols = Vec::new();
            $(
                sols.extend($year());
            )*
            sols
        }
    }
}

library!(
    y2026 { d01, d02 }
);
