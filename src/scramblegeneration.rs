use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    U, // Up
    D, // Down
    L, // Left
    R, // Right
    F, // Front
    B, // Back
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveVariation {
    Normal,
    Prime,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrambleMove {
    pub mv: Move,
    pub variation: MoveVariation,
}

impl ScrambleMove {
    pub fn to_string(&self) -> String {
        let move_str = match self.mv {
            Move::U => "U",
            Move::D => "D",
            Move::L => "L",
            Move::R => "R",
            Move::F => "F",
            Move::B => "B",
        };

        let variation_str = match self.variation {
            MoveVariation::Normal => "",
            MoveVariation::Prime => "'",
            MoveVariation::Double => "2",
        };

        format!("{}{}", move_str, variation_str)
    }
}

// Function to check if two moves cancel each other
pub(crate) fn moves_cancel(m1: &ScrambleMove, m2: &ScrambleMove) -> bool {
    m1.mv == m2.mv
        && ((m1.variation == MoveVariation::Normal && m2.variation == MoveVariation::Prime)
            || (m1.variation == MoveVariation::Prime && m2.variation == MoveVariation::Normal)
            || (m1.variation == MoveVariation::Double && m2.variation == MoveVariation::Double))
}

// Function to check if two moves are the same (even with different variations)
pub(crate) fn moves_repeat(m1: &ScrambleMove, m2: &ScrambleMove) -> bool {
    m1.mv == m2.mv
}

// Function to check if two moves affect opposite faces (like U and D, or L and R)
pub(crate) fn are_opposite_faces(m1: &Move, m2: &Move) -> bool {
    match (m1, m2) {
        (Move::U, Move::D) | (Move::D, Move::U) => true,
        (Move::L, Move::R) | (Move::R, Move::L) => true,
        (Move::F, Move::B) | (Move::B, Move::F) => true,
        _ => false,
    }
}

// Function to generate a random scramble sequence without repeats, cancellations, or alternating opposite-face moves
pub fn generate_scramble(length: usize) -> Vec<ScrambleMove> {
    let moves = vec![Move::U, Move::D, Move::L, Move::R, Move::F, Move::B];
    let variations = vec![
        MoveVariation::Normal,
        MoveVariation::Prime,
        MoveVariation::Double,
    ];

    let mut rng = thread_rng();
    let mut scramble: Vec<ScrambleMove> = Vec::new();

    for _ in 0..length {
        loop {
            let mv = *moves.choose(&mut rng).unwrap();
            let variation = *variations.choose(&mut rng).unwrap();
            let new_move = ScrambleMove { mv, variation };

            // Check if the new move repeats or cancels the last move
            if let Some(last_move) = scramble.last() {
                if moves_repeat(last_move, &new_move) || moves_cancel(last_move, &new_move) {
                    continue;
                }

                // Check for a pattern like U D U or D U D' for opposite faces
                if scramble.len() > 1 {
                    let second_last_move = scramble[scramble.len() - 2];
                    if are_opposite_faces(&second_last_move.mv, &new_move.mv)
                        && are_opposite_faces(&last_move.mv, &new_move.mv)
                    {
                        continue;
                    }
                }
            }

            scramble.push(new_move);
            break;
        }
    }

    scramble
}

pub fn generate_scramble_string(length: usize) -> String {
    let scramble = generate_scramble(length);
    let mut scramble_str = String::new();

    for mv in scramble {
        scramble_str.push_str(&mv.to_string());
        scramble_str.push(' ');
    }

    scramble_str
}
