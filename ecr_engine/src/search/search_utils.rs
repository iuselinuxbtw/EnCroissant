use crate::board::Board;
use std::fmt::Display;
use trees::{tr, Node, Tree};

pub fn search(depth: u8) {
    //TODO: Implement a Graph containing either the full board oor a minimal version of the board(or the move).
}

/// Prints the tree fens from a given nodet o a string.
pub fn tree_to_string<T: Display>(node: &Node<T>) -> String {
    if node.has_no_child() {
        node.data().to_string()
    } else {
        format!(
            "{}, ({})",
            node.data(),
            node.iter()
                .fold(String::new(), |s, c| s + &tree_to_string(c) + &" ")
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
        let mut tree = tr(board.clone());
        let mut root = tree.root_mut();

        // Execute every possible move in the variation vector.
        let mut variations = vec![];
        for moves in root.data().get_pseudo_legal_moves(board.to_move) {
            for m in moves.basic_move {
                let mut cloned_board = board.clone();
                cloned_board.r#move(moves.from, &m);
                variations.push(cloned_board);
            }
        }
        // Add all variations to the tree
        for variation in variations {
            root.push_back(tr(variation));
        }

        // Print the tree sorted by preorder
        assert_eq!(32, root.data().pieces.len());
        println!("{}", tree_to_string(tree.root()));
        println!("{}", tree);
    }
}
