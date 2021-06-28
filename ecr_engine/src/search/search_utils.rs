use crate::board::Board;
use std::fmt::Display;
use trees::{tr, Node, Tree};

pub fn search(board: &Board, depth: u8) -> Tree<Board> {
    let mut tree = tr(board.clone());
    let mut root = tree.root_mut();

    // Execute every possible move in the variation vector.
    let mut variations = vec![];
    for moves in root.data().get_pseudo_legal_moves(board.to_move) {
        for m in moves.basic_move {
            let mut cloned_board = board.clone();
            cloned_board.do_blunder(moves.from, &m);
            variations.push(cloned_board);
        }
    }
    // Add all variations to the tree
    for variation in variations {
        root.push_back(tr(variation));
    }

    tree
}

/*
/// Used for recursion necessary for depth-search in search.
fn search_util(root: Tree<Board>, depth: u8) {
    //TODO
}*/

/// Prints the tree fens from a given node o a string.
pub fn tree_to_string<T: Display>(node: &Node<T>) -> String {
    if node.has_no_child() {
        node.data().to_string()
    } else {
        format!(
            "{}, ({})",
            node.data(),
            node.iter()
                .fold(String::new(), |s, c| s + &tree_to_string(c) + " ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        // Create a default board
        let board = Board::default();
        let tree = search(&board, 1);
        assert_eq!(20, tree.degree());
    }
}
