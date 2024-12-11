use criterion::{black_box, criterion_group, criterion_main, Criterion};

use matrix::Matrix;
use matrix_multiplication::matrix_multiplication;

use ndarray::{Array, Ix2};

fn benchmark_matrix_multiplication(crit: &mut Criterion) {
    let initial_value = 2.5f64;

    let mut group = crit.benchmark_group("matrix_multiplication");
    group.measurement_time(std::time::Duration::new(20, 0));
    group.sample_size(10);
    for size in (100..=1000).step_by(100) {
        let m = size;
        let n = size;
        let k = size;

        let mut c = Matrix::new(m, n, 0.0f64);
        let a = Matrix::new(m, k, initial_value);
        let b = Matrix::new(k, n, initial_value);

        group.bench_function(format!("{}x{}x{}", m, n, k), |block| {
            block.iter(|| {
                matrix_multiplication(black_box(&mut c), black_box(&a), black_box(&b)).unwrap();
            })
        });
    }

    group.finish();
    let mut group = crit.benchmark_group("ndarray::dot");
    group.measurement_time(std::time::Duration::new(10, 0));
    group.sample_size(10);
    for size in (100..=1000).step_by(100) {
        let m = size;
        let n = size;
        let k = size;

        let mut c = Array::<f64, Ix2>::from_elem((m,n), 0.0f64);
        let a = Array::<f64, Ix2>::from_elem((m,k), initial_value);
        let b = Array::<f64, Ix2>::from_elem((k,n), initial_value);

        group.bench_function(format!("{}x{}x{}", m, n, k), |block| {
            block.iter(|| {
                c = a.dot(&b);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_matrix_multiplication);
criterion_main!(benches);
