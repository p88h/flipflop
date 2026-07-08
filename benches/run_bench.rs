use criterion::{criterion_group, criterion_main, Criterion};

fn read_input(year: i32, puzzle: i32, part: i32) -> String {
    let path = format!("input/flipflop_codes_{year}_{puzzle:02}_{part}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let solutions = flipflop_codes::e2025();
    for sol in solutions {
        let input1 = read_input(sol.year, sol.puzzle, 1);
        let input2 = read_input(sol.year, sol.puzzle, 2);
        let input3: String = read_input(sol.year, sol.puzzle, 3);
        let name = format!("{}.{}", sol.year, sol.puzzle);
        let mut group = c.benchmark_group(&name);
        group.bench_function("part1", |b| b.iter(|| (sol.part1)(&input1)));
        group.bench_function("part2", |b| b.iter(|| (sol.part2)(&input2)));
        group.bench_function("part3", |b| b.iter(|| (sol.part3)(&input3)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
