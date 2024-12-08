use criterion::{black_box, criterion_group, criterion_main, Criterion};

use matrix::Matrix;
use matrix_multiplication::matrix_multiplication;

fn benchmark_matrix_multiplication(crit: &mut Criterion) {
    let m = 1000;
    let n = 1000;
    let k = 1000;
    let initial_value = 2.5f64;

    let mut c = Matrix::new(m, n, 0.0f64);
    let a = Matrix::new(m, k, initial_value);
    let b = Matrix::new(k, n, initial_value);

    let mut group = crit.benchmark_group("matrix_multiplication-1500x1500x1500");
    group.measurement_time(std::time::Duration::new(10, 0));
    group.sample_size(10);

    group.bench_function("Matrix::set", |block| {
        block.iter(|| {
            matrix_multiplication(black_box(&mut c), black_box(&a), black_box(&b)).unwrap();
        })
    });
        
    group.finish();
}
criterion_group!(benches, benchmark_matrix_multiplication);
criterion_main!(benches);
