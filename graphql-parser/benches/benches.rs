use criterion::{criterion_group, criterion_main, Criterion};
use graphql_parser::*;

fn load_graphql(path_list: &[&str]) -> Vec<String> {
    path_list
        .iter()
        .filter_map(|path| std::fs::read_dir(path).ok())
        .flat_map(|dir| dir)
        .flat_map(|path| path.into_iter())
        .map(|entry| entry.path())
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let type_system_list = load_graphql(&[
        "tests/type_system/directives",
        "tests/type_system/enums",
        "tests/type_system/input_objects",
        "tests/type_system/input_values",
        "tests/type_system/interfaces",
        "tests/type_system/objects",
        "tests/type_system/scalars",
        "tests/type_system/schema",
        "tests/type_system/unions",
    ]);
    c.bench_function("type_system", |b| {
        b.iter(|| {
            for source in type_system_list.iter() {
                let _ = parse_type_system(source.as_str());
            }
        });
    });

    let executable_list = load_graphql(&[
        "tests/executable/fragment",
        "tests/executable/query",
        "tests/executable/mutation",
        "tests/executable/subscription",
    ]);
    c.bench_function("executable", |b| {
        b.iter(|| {
            for source in executable_list.iter() {
                let _ = parse_executable(source.as_str());
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
