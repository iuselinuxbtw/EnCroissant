use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecr_engine::board;
use ecr_engine::board::Board;
use ecr_engine::pieces::move_gen::*;
use ecr_engine::pieces::{BoardPiece, PieceColor};
use ecr_shared::pieces::PieceType;

/// Generates a light-colored piece of the given type for every square on the board.
fn generate_pieces_of_type(piece_type: PieceType) -> Vec<BoardPiece> {
    let mut result: Vec<BoardPiece> = vec![];
    for x in 0..7 {
        for y in 0..=7 {
            result.push(BoardPiece::new_from_type(
                piece_type,
                (x, y).into(),
                PieceColor::Light,
            ));
        }
    }
    result
}

fn get_moves_of_pieces(pieces: Vec<BoardPiece>, board: &board::Board) {
    for piece in &pieces {
        piece.get_piece().get_pseudo_legal_moves(
            board,
            &piece.get_coordinate(),
            PieceColor::Light,
            false,
        );
    }
}

fn bench_pawn_moves(b: &mut Criterion) {
    // TODO: En_passant(Though we should get some board for this...)
    let pieces = generate_pieces_of_type(PieceType::Pawn);
    let default_board = Board::default();
    b.bench_function("Pawn moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_rook_moves(b: &mut Criterion) {
    let pieces = generate_pieces_of_type(PieceType::Rook);
    let default_board = Board::default();
    b.bench_function("Rook moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_bishop_moves(b: &mut Criterion) {
    let pieces = generate_pieces_of_type(PieceType::Bishop);
    let default_board = Board::default();
    b.bench_function("Bishop moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_king_moves(b: &mut Criterion) {
    let pieces = generate_pieces_of_type(PieceType::King);
    let default_board = Board::default();
    b.bench_function("King moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_knight_moves(b: &mut Criterion) {
    let pieces = generate_pieces_of_type(PieceType::Knight);
    let default_board = Board::default();
    b.bench_function("Knight moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_queen_moves(b: &mut Criterion) {
    let pieces = generate_pieces_of_type(PieceType::Queen);
    let default_board = Board::default();
    b.bench_function("Queen moves", |c| {
        c.iter(|| {
            get_moves_of_pieces(pieces.clone(), &default_board);
        })
    });
}

fn bench_evaluation(b: &mut Criterion) {
    let default_board = Board::default();
    b.bench_function("Evaluation", |c| {
        c.iter(|| {
            black_box(default_board.eval_board());
        })
    });
}

fn bench_move(b: &mut Criterion) {
    let default_board = Board::default();
    b.bench_function("Move", |c| {
        c.iter(|| {
            // The best opening move known to mankind
            default_board.clone().r#move(
                black_box((5, 1).into()),
                &BasicMove {
                    to: (5, 2).into(),
                    capture: None,
                },
            )
        })
    });
}

fn bench_get_castle_moves(b: &mut Criterion) {
    let default_board = Board::default();
    b.bench_function("Get Castle Moves", |c| {
        c.iter(|| {
            get_castle_moves(
                black_box(default_board.get_castle_state()),
                &PieceColor::Light,
                &default_board,
            )
        })
    });
}

// This should probably be split into multiple groups
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_pawn_moves, bench_rook_moves, bench_bishop_moves, bench_king_moves, bench_knight_moves, bench_queen_moves, bench_evaluation, bench_move, bench_get_castle_moves
}

criterion_main!(benches);
