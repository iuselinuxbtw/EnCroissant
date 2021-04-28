/// Holds information whether castling is allowed on the specific sides.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardCastleState {
    /// Can light castle on king side?
    pub light_king_side: bool,
    /// Can light castle on queen side?
    pub light_queen_side: bool,
    /// Can dark castle on king side?
    pub dark_king_side: bool,
    /// Can dark castle on queen side?
    pub dark_queen_side: bool,
}

impl Default for BoardCastleState {
    /// By default, every castle action is possible.
    fn default() -> Self {
        BoardCastleState {
            light_king_side: true,
            light_queen_side: true,
            dark_king_side: true,
            dark_queen_side: true,
        }
    }
}

impl BoardCastleState {
    /// Returns if any castle action is still allowed.
    pub fn is_any_possible(&self) -> bool {
        self.light_king_side || self.light_queen_side || self.dark_king_side || self.dark_queen_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board_castle_state {
        use super::*;

        #[test]
        fn test_is_any_possible() {
            assert!(!BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: false,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: false,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: true,
                dark_king_side: false,
                dark_queen_side: false,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: true,
                dark_queen_side: false,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: false,
                light_queen_side: false,
                dark_king_side: false,
                dark_queen_side: true,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: false,
                dark_king_side: true,
                dark_queen_side: false,
            }
                .is_any_possible());
            assert!(BoardCastleState {
                light_king_side: true,
                light_queen_side: true,
                dark_king_side: true,
                dark_queen_side: true,
            }
                .is_any_possible());
        }

        #[test]
        fn test_default() {
            assert_eq!(
                BoardCastleState {
                    light_king_side: true,
                    light_queen_side: true,
                    dark_king_side: true,
                    dark_queen_side: true,
                },
                BoardCastleState::default()
            );
        }
    }
}