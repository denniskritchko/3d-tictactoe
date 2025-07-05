use bevy::prelude::*;
use crate::ai::MCTSAi;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Human,
    AI,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Human,
    AI,
}

#[derive(Resource)]
pub struct GameState {
    pub board: [[[CellState; 3]; 3]; 3],
    pub current_player: Player,
    pub game_over: bool,
    pub winner: Option<Player>,
    pub ai: MCTSAi,
    pub selected_cube: Option<(usize, usize, usize)>,
    pub last_move: Option<(usize, usize, usize)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [[[CellState::Empty; 3]; 3]; 3],
            current_player: Player::Human,
            game_over: false,
            winner: None,
            ai: MCTSAi::new(),
            selected_cube: None,
            last_move: None,
        }
    }
}

impl GameState {
    pub fn make_move(&mut self, x: usize, y: usize, z: usize) -> bool {
        if self.game_over || self.board[x][y][z] != CellState::Empty {
            return false;
        }

        match self.current_player {
            Player::Human => self.board[x][y][z] = CellState::Human,
            Player::AI => self.board[x][y][z] = CellState::AI,
        }

        // Track the last move for animations
        self.last_move = Some((x, y, z));

        if self.check_winner() {
            self.game_over = true;
            self.winner = Some(self.current_player);
        } else if self.is_board_full() {
            self.game_over = true;
            self.winner = None; // Draw
        } else {
            self.current_player = match self.current_player {
                Player::Human => Player::AI,
                Player::AI => Player::Human,
            };
        }

        true
    }

    pub fn check_winner(&self) -> bool {
        // Check all possible winning lines in 3D
        // Lines along X axis
        for y in 0..3 {
            for z in 0..3 {
                if self.check_line([(0, y, z), (1, y, z), (2, y, z)]) {
                    return true;
                }
            }
        }

        // Lines along Y axis
        for x in 0..3 {
            for z in 0..3 {
                if self.check_line([(x, 0, z), (x, 1, z), (x, 2, z)]) {
                    return true;
                }
            }
        }

        // Lines along Z axis
        for x in 0..3 {
            for y in 0..3 {
                if self.check_line([(x, y, 0), (x, y, 1), (x, y, 2)]) {
                    return true;
                }
            }
        }

        // Face diagonals on XY planes
        for z in 0..3 {
            if self.check_line([(0, 0, z), (1, 1, z), (2, 2, z)]) ||
               self.check_line([(0, 2, z), (1, 1, z), (2, 0, z)]) {
                return true;
            }
        }

        // Face diagonals on XZ planes
        for y in 0..3 {
            if self.check_line([(0, y, 0), (1, y, 1), (2, y, 2)]) ||
               self.check_line([(0, y, 2), (1, y, 1), (2, y, 0)]) {
                return true;
            }
        }

        // Face diagonals on YZ planes
        for x in 0..3 {
            if self.check_line([(x, 0, 0), (x, 1, 1), (x, 2, 2)]) ||
               self.check_line([(x, 0, 2), (x, 1, 1), (x, 2, 0)]) {
                return true;
            }
        }

        // 3D diagonals (corner to corner)
        if self.check_line([(0, 0, 0), (1, 1, 1), (2, 2, 2)]) ||
           self.check_line([(0, 0, 2), (1, 1, 1), (2, 2, 0)]) ||
           self.check_line([(0, 2, 0), (1, 1, 1), (2, 0, 2)]) ||
           self.check_line([(0, 2, 2), (1, 1, 1), (2, 0, 0)]) {
            return true;
        }

        false
    }

    fn check_line(&self, positions: [(usize, usize, usize); 3]) -> bool {
        let cells = [
            self.board[positions[0].0][positions[0].1][positions[0].2],
            self.board[positions[1].0][positions[1].1][positions[1].2],
            self.board[positions[2].0][positions[2].1][positions[2].2],
        ];

        cells[0] != CellState::Empty && cells[0] == cells[1] && cells[1] == cells[2]
    }

    fn is_board_full(&self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if self.board[x][y][z] == CellState::Empty {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_empty_positions(&self) -> Vec<(usize, usize, usize)> {
        let mut positions = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if self.board[x][y][z] == CellState::Empty {
                        positions.push((x, y, z));
                    }
                }
            }
        }
        positions
    }

    pub fn reset(&mut self) {
        self.board = [[[CellState::Empty; 3]; 3]; 3];
        self.current_player = Player::Human;
        self.game_over = false;
        self.winner = None;
        self.selected_cube = None;
        self.last_move = None;
    }
} 