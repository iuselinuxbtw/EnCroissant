use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecr_chess::pieces::move_gen::*;
use ecr_chess::pieces::PieceColor;

fn bench_pawn_moves(b: &mut Criterion) {
    b.bench_function("diagonal_moves", |c| {
        c.iter(|| {
            pawn_moves(
                &black_box((5, 6).into()),
                &PieceColor::Light,
                &Default::default(),
                false,
            )
        })
    });
}
fn bench_linear_moves(b: &mut Criterion) {
    b.bench_function("linear_moves", |c| {
        c.iter(|| {
            linear_moves(
                &black_box((5, 6).into()),
                &Default::default(),
                &PieceColor::Light,
            )
        })
    });
}
fn bench_diagonal_moves(b: &mut Criterion) {
    b.bench_function("diagonal_moves", |c| {
        c.iter(|| {
            diagonal_moves(
                &black_box((5, 6).into()),
                &PieceColor::Light,
                &Default::default(),
            )
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_pawn_moves, bench_linear_moves, bench_diagonal_moves()
}
criterion_main!(benches);
