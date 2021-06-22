use trees::tr;

pub fn search() {
    //TODO: Implement a Graph containing either the full board oor a minimal version of the board(or the move).
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use super::*;

    #[test]
    fn test_search() {
        let mut board = Board::default();
        let mut tree = tr(board.clone());
        let mut root = tree.root_mut();
        let mut variations = vec![];
        for moves in root.data().get_all_pseudo_legal_moves() {
            for m in moves.basic_move {
                let mut cloned_board = board.clone();
                cloned_board.r#move(moves.from, &m);
                variations.push(cloned_board);
            }
        }
        for variation in variations {
            root.push_back(tr(variation));
        }
    }
}
