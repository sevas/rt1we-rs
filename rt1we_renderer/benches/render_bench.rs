use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rt1we_renderer::geometry::Vec3;
use rt1we_renderer::render::{render, render_parallel};

const MAX_DEPTH: usize = 10;
const SAMPLES_PER_PIXEL: usize = 8;
const RESOLUTIONS: &[(usize, usize)] = &[(64, 64), (128, 128), (256, 256)];

fn bench_render_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_single_core");
    group.sample_size(10);

    for &(width, height) in RESOLUTIONS {
        let position = Vec3::new(0.0, 0.0, 0.0);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{width}x{height}")),
            &(width, height),
            |b, &(width, height)| {
                b.iter(|| render(width, height, MAX_DEPTH, SAMPLES_PER_PIXEL, &position, false));
            },
        );
    }
    group.finish();
}

fn bench_render_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_parallel");
    group.sample_size(10);

    for &(width, height) in RESOLUTIONS {
        let position = Vec3::new(0.0, 0.0, 0.0);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{width}x{height}")),
            &(width, height),
            |b, &(width, height)| {
                b.iter(|| {
                    render_parallel(width, height, MAX_DEPTH, SAMPLES_PER_PIXEL, &position, false)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_render_single, bench_render_parallel);
criterion_main!(benches);
