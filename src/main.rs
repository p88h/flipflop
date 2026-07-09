use std::time::Instant;

fn read_input(year: i32, puzzle: i32, _part: i32) -> String {
    let path = format!("input/flipflop_codes_{year}_{puzzle:02}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

fn maybe_parse_arg(args: &Vec<String>, pos: usize, default: i32) -> i32 {
    if args.len() <= pos {
        return default;
    }
    if args[pos].is_empty() || args[pos] == "all" {
        return -1;
    }
    if let Some(first_char) = args[pos].chars().next() {
        if first_char.is_alphabetic() {
            return args[pos][1..].parse().unwrap_or(-1);
        }
    }
    return args[pos].parse().unwrap_or(-1);
}

fn run_part(year: i32, puzzle: i32, part: i32, func: fn(&str) -> String) -> (String, String) {
    let input = read_input(year, puzzle, part);
    let start = Instant::now();
    let result = func(&input);
    for _ in 0..9 {
        let other = func(&input);
        if other != result {
            panic!(
                "Inconsistent results for year {}, puzzle {}, part {}: '{}' vs '{}'",
                year, puzzle, part, result, other
            );
        }
    }
    let duration = start.elapsed() / 10;
    // auto format duration
    if duration.as_nanos() < 1_000 {
        (result, format!("({} ns)", duration.as_nanos()))
    } else if duration.as_micros() < 1_000 {
        (result, format!("({} µs)", duration.as_micros()))
    } else if duration.as_millis() < 1_000 {
        (result, format!("({} ms)", duration.as_millis()))
    } else {
        (result, format!("({} s)", duration.as_secs_f64()))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let solutions = flipflop_codes::solutions();
    let year = maybe_parse_arg(&args, 1, solutions[solutions.len() - 1].year);
    let puzzle = maybe_parse_arg(&args, 2, -1);
    for sol in solutions {
        if (year != -1 && sol.year != year) || (puzzle != -1 && sol.puzzle != puzzle) {
            continue;
        }
        let (r1, d1) = run_part(sol.year, sol.puzzle, 1, sol.part1);
        let (r2, d2) = run_part(sol.year, sol.puzzle, 2, sol.part2);
        let (r3, d3) = run_part(sol.year, sol.puzzle, 3, sol.part3);
        let label = if sol.year >= 1000 {
            format!("year {} puzzle {:02}", sol.year, sol.puzzle)
        } else {
            format!("Story {:04} puzzle {:02}", sol.year, sol.puzzle)
        };
        println!("{label} Part 1: {r1:16} {d1:8}  Part 2: {r2:16} {d2:8}  Part 3: {r3:16} {d3:8}");
    }
}
