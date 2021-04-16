use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecr_chess::board::Board;
use ecr_chess::pieces::move_gen::*;
use ecr_chess::pieces::PieceColor;

// TODO: Do these with other values. Maybe iterate through all values.
fn bench_pawn_moves(b: &mut Criterion) {
    b.bench_function("Pawn moves", |c| {
        c.iter(|| {
            pawn_moves(
                &black_box((5, 6).into()),
                &Default::default(),
                &PieceColor::Light,
                false,
            )
        })
    });
}

fn bench_linear_moves(b: &mut Criterion) {
    b.bench_function("Linear moves", |c| {
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
    b.bench_function("Diagonal moves", |c| {
        c.iter(|| {
            diagonal_moves(
                &black_box((5, 6).into()),
                &Default::default(),
                &PieceColor::Light,
            )
        })
    });
}

fn bench_king_moves(b: &mut Criterion) {
    b.bench_function("King moves", |c| {
        c.iter(|| {
            king_moves(
                &black_box((5, 6).into()),
                &Default::default(),
                &PieceColor::Light,
            )
        })
    });
}

fn bench_knight_moves(b: &mut Criterion) {
    b.bench_function("Knight moves", |c| {
        c.iter(|| {
            diagonal_moves(
                &black_box((5, 6).into()),
                &Default::default(),
                &PieceColor::Light,
            )
        })
    });
}

fn bench_evaluation(b: &mut Criterion) {
    let default_board = Board::default();
    b.bench_function("Evaluation", |c| {
        c.iter(|| {
            default_board.eval_board();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_pawn_moves, bench_linear_moves, bench_diagonal_moves, bench_king_moves, bench_knight_moves
}
criterion_main!(benches);
