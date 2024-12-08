use criterion::{black_box, criterion_group, criterion_main, Criterion};

use matrix::Matrix;

fn benchmark_new(c: &mut Criterion) {
    let nrows = 1000;
    let ncols = 1000;
    let initial_value = 2.5f64;

    c.bench_function("Matrix::new", |b| {
        b.iter(|| {
            Matrix::new(
                black_box(nrows),
                black_box(ncols),
                black_box(initial_value),
                );
        })
    });
}

fn benchmark_set(c: &mut Criterion) {
    let nrows = 1000;
    let ncols = 1000;
    let initial_value = 2.5f64;
    let final_value = 3.5f64;

    let mut matrix = Matrix::new(nrows, ncols, initial_value);

    let mut group = c.benchmark_group("Matrix::set");
    group.measurement_time(std::time::Duration::new(10, 0));
    group.bench_function("Matrix::set", |b| {
        b.iter(|| {
            for row in 0..nrows {
                for col in 0..ncols {
                    matrix
                        .set(
                            black_box(row),
                            black_box(col),
                            black_box(final_value),
                        )
                        .unwrap();
                }
            }
        })
    });
    group.finish();
}

fn benchmark_get(c: &mut Criterion) {
    let nrows = 1000;
    let ncols = 1000;
    let initial_value = 2.5f64;

    let matrix = Matrix::new(nrows, ncols, initial_value);

    c.bench_function("Matrix::get", |b| {
        b.iter(|| {
            for row in 0..nrows {
                for col in 0..ncols {
                    matrix
                        .get(
                            black_box(row),
                            black_box(col),
                        )
                        .unwrap();
                }
            }
        })
    });
}

criterion_group!(benches, benchmark_new, benchmark_set, benchmark_get);
criterion_main!(benches);
