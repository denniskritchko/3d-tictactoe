use rand::Rng;
use std::collections::HashMap;
use crate::game::{GameState, Player, CellState};

#[derive(Clone)]
pub struct MCTSNode {
    pub state: [[[CellState; 3]; 3]; 3],
    pub current_player: Player,
    pub parent: Option<Box<MCTSNode>>,
    pub children: Vec<MCTSNode>,
    pub visits: u32,
    pub wins: u32,
    pub last_move: Option<(usize, usize, usize)>,
}

impl MCTSNode {
    pub fn new(state: [[[CellState; 3]; 3]; 3], current_player: Player) -> Self {
        Self {
            state,
            current_player,
            parent: None,
            children: Vec::new(),
            visits: 0,
            wins: 0,
            last_move: None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.check_winner().is_some() || self.is_board_full()
    }

    pub fn check_winner(&self) -> Option<Player> {
        // Check all possible winning lines in 3D (same logic as GameState)
        // Lines along X axis
        for y in 0..3 {
            for z in 0..3 {
                if self.check_line([(0, y, z), (1, y, z), (2, y, z)]) {
                    return Some(self.get_winner_from_line([(0, y, z), (1, y, z), (2, y, z)]));
                }
            }
        }

        // Lines along Y axis
        for x in 0..3 {
            for z in 0..3 {
                if self.check_line([(x, 0, z), (x, 1, z), (x, 2, z)]) {
                    return Some(self.get_winner_from_line([(x, 0, z), (x, 1, z), (x, 2, z)]));
                }
            }
        }

        // Lines along Z axis
        for x in 0..3 {
            for y in 0..3 {
                if self.check_line([(x, y, 0), (x, y, 1), (x, y, 2)]) {
                    return Some(self.get_winner_from_line([(x, y, 0), (x, y, 1), (x, y, 2)]));
                }
            }
        }

        // Face diagonals on XY planes
        for z in 0..3 {
            if self.check_line([(0, 0, z), (1, 1, z), (2, 2, z)]) {
                return Some(self.get_winner_from_line([(0, 0, z), (1, 1, z), (2, 2, z)]));
            }
            if self.check_line([(0, 2, z), (1, 1, z), (2, 0, z)]) {
                return Some(self.get_winner_from_line([(0, 2, z), (1, 1, z), (2, 0, z)]));
            }
        }

        // Face diagonals on XZ planes
        for y in 0..3 {
            if self.check_line([(0, y, 0), (1, y, 1), (2, y, 2)]) {
                return Some(self.get_winner_from_line([(0, y, 0), (1, y, 1), (2, y, 2)]));
            }
            if self.check_line([(0, y, 2), (1, y, 1), (2, y, 0)]) {
                return Some(self.get_winner_from_line([(0, y, 2), (1, y, 1), (2, y, 0)]));
            }
        }

        // Face diagonals on YZ planes
        for x in 0..3 {
            if self.check_line([(x, 0, 0), (x, 1, 1), (x, 2, 2)]) {
                return Some(self.get_winner_from_line([(x, 0, 0), (x, 1, 1), (x, 2, 2)]));
            }
            if self.check_line([(x, 0, 2), (x, 1, 1), (x, 2, 0)]) {
                return Some(self.get_winner_from_line([(x, 0, 2), (x, 1, 1), (x, 2, 0)]));
            }
        }

        // 3D diagonals (corner to corner)
        if self.check_line([(0, 0, 0), (1, 1, 1), (2, 2, 2)]) {
            return Some(self.get_winner_from_line([(0, 0, 0), (1, 1, 1), (2, 2, 2)]));
        }
        if self.check_line([(0, 0, 2), (1, 1, 1), (2, 2, 0)]) {
            return Some(self.get_winner_from_line([(0, 0, 2), (1, 1, 1), (2, 2, 0)]));
        }
        if self.check_line([(0, 2, 0), (1, 1, 1), (2, 0, 2)]) {
            return Some(self.get_winner_from_line([(0, 2, 0), (1, 1, 1), (2, 0, 2)]));
        }
        if self.check_line([(0, 2, 2), (1, 1, 1), (2, 0, 0)]) {
            return Some(self.get_winner_from_line([(0, 2, 2), (1, 1, 1), (2, 0, 0)]));
        }

        None
    }

    fn check_line(&self, positions: [(usize, usize, usize); 3]) -> bool {
        let cells = [
            self.state[positions[0].0][positions[0].1][positions[0].2],
            self.state[positions[1].0][positions[1].1][positions[1].2],
            self.state[positions[2].0][positions[2].1][positions[2].2],
        ];

        cells[0] != CellState::Empty && cells[0] == cells[1] && cells[1] == cells[2]
    }

    fn get_winner_from_line(&self, positions: [(usize, usize, usize); 3]) -> Player {
        let cell = self.state[positions[0].0][positions[0].1][positions[0].2];
        match cell {
            CellState::Human => Player::Human,
            CellState::AI => Player::AI,
            CellState::Empty => panic!("Empty cell shouldn't be a winner"),
        }
    }

    fn is_board_full(&self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if self.state[x][y][z] == CellState::Empty {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_possible_moves(&self) -> Vec<(usize, usize, usize)> {
        let mut moves = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if self.state[x][y][z] == CellState::Empty {
                        moves.push((x, y, z));
                    }
                }
            }
        }
        moves
    }

    pub fn make_move(&self, x: usize, y: usize, z: usize) -> MCTSNode {
        let mut new_state = self.state;
        match self.current_player {
            Player::Human => new_state[x][y][z] = CellState::Human,
            Player::AI => new_state[x][y][z] = CellState::AI,
        }

        let next_player = match self.current_player {
            Player::Human => Player::AI,
            Player::AI => Player::Human,
        };

        let mut node = MCTSNode::new(new_state, next_player);
        node.last_move = Some((x, y, z));
        node
    }

    pub fn expand(&mut self) {
        let moves = self.get_possible_moves();
        for (x, y, z) in moves {
            let child = self.make_move(x, y, z);
            self.children.push(child);
        }
    }

    pub fn uct_value(&self, exploration_param: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }

        let win_rate = self.wins as f64 / self.visits as f64;
        let exploration = exploration_param * (2.0 * (self.visits as f64).ln() / self.visits as f64).sqrt();
        win_rate + exploration
    }

    pub fn select_best_child(&self, exploration_param: f64) -> usize {
        let mut best_value = f64::NEG_INFINITY;
        let mut best_index = 0;

        for (i, child) in self.children.iter().enumerate() {
            let value = child.uct_value(exploration_param);
            if value > best_value {
                best_value = value;
                best_index = i;
            }
        }

        best_index
    }

    pub fn simulate(&self) -> Player {
        let mut rng = rand::thread_rng();
        let mut current_state = self.state;
        let mut current_player = self.current_player;

        loop {
            if let Some(winner) = self.check_winner_for_state(&current_state) {
                return winner;
            }

            let moves = self.get_possible_moves_for_state(&current_state);
            if moves.is_empty() {
                // Draw - return random player
                return if rng.gen_bool(0.5) { Player::Human } else { Player::AI };
            }

            let (x, y, z) = moves[rng.gen_range(0..moves.len())];
            match current_player {
                Player::Human => current_state[x][y][z] = CellState::Human,
                Player::AI => current_state[x][y][z] = CellState::AI,
            }

            current_player = match current_player {
                Player::Human => Player::AI,
                Player::AI => Player::Human,
            };
        }
    }

    fn check_winner_for_state(&self, state: &[[[CellState; 3]; 3]; 3]) -> Option<Player> {
        // Check all possible winning lines in 3D (same logic as GameState)
        // Lines along X axis
        for y in 0..3 {
            for z in 0..3 {
                if self.check_line_for_state(state, [(0, y, z), (1, y, z), (2, y, z)]) {
                    return Some(self.get_winner_from_line_for_state(state, [(0, y, z), (1, y, z), (2, y, z)]));
                }
            }
        }

        // Lines along Y axis
        for x in 0..3 {
            for z in 0..3 {
                if self.check_line_for_state(state, [(x, 0, z), (x, 1, z), (x, 2, z)]) {
                    return Some(self.get_winner_from_line_for_state(state, [(x, 0, z), (x, 1, z), (x, 2, z)]));
                }
            }
        }

        // Lines along Z axis
        for x in 0..3 {
            for y in 0..3 {
                if self.check_line_for_state(state, [(x, y, 0), (x, y, 1), (x, y, 2)]) {
                    return Some(self.get_winner_from_line_for_state(state, [(x, y, 0), (x, y, 1), (x, y, 2)]));
                }
            }
        }

        // Face diagonals on XY planes
        for z in 0..3 {
            if self.check_line_for_state(state, [(0, 0, z), (1, 1, z), (2, 2, z)]) {
                return Some(self.get_winner_from_line_for_state(state, [(0, 0, z), (1, 1, z), (2, 2, z)]));
            }
            if self.check_line_for_state(state, [(0, 2, z), (1, 1, z), (2, 0, z)]) {
                return Some(self.get_winner_from_line_for_state(state, [(0, 2, z), (1, 1, z), (2, 0, z)]));
            }
        }

        // Face diagonals on XZ planes
        for y in 0..3 {
            if self.check_line_for_state(state, [(0, y, 0), (1, y, 1), (2, y, 2)]) {
                return Some(self.get_winner_from_line_for_state(state, [(0, y, 0), (1, y, 1), (2, y, 2)]));
            }
            if self.check_line_for_state(state, [(0, y, 2), (1, y, 1), (2, y, 0)]) {
                return Some(self.get_winner_from_line_for_state(state, [(0, y, 2), (1, y, 1), (2, y, 0)]));
            }
        }

        // Face diagonals on YZ planes
        for x in 0..3 {
            if self.check_line_for_state(state, [(x, 0, 0), (x, 1, 1), (x, 2, 2)]) {
                return Some(self.get_winner_from_line_for_state(state, [(x, 0, 0), (x, 1, 1), (x, 2, 2)]));
            }
            if self.check_line_for_state(state, [(x, 0, 2), (x, 1, 1), (x, 2, 0)]) {
                return Some(self.get_winner_from_line_for_state(state, [(x, 0, 2), (x, 1, 1), (x, 2, 0)]));
            }
        }

        // 3D diagonals (corner to corner)
        if self.check_line_for_state(state, [(0, 0, 0), (1, 1, 1), (2, 2, 2)]) {
            return Some(self.get_winner_from_line_for_state(state, [(0, 0, 0), (1, 1, 1), (2, 2, 2)]));
        }
        if self.check_line_for_state(state, [(0, 0, 2), (1, 1, 1), (2, 2, 0)]) {
            return Some(self.get_winner_from_line_for_state(state, [(0, 0, 2), (1, 1, 1), (2, 2, 0)]));
        }
        if self.check_line_for_state(state, [(0, 2, 0), (1, 1, 1), (2, 0, 2)]) {
            return Some(self.get_winner_from_line_for_state(state, [(0, 2, 0), (1, 1, 1), (2, 0, 2)]));
        }
        if self.check_line_for_state(state, [(0, 2, 2), (1, 1, 1), (2, 0, 0)]) {
            return Some(self.get_winner_from_line_for_state(state, [(0, 2, 2), (1, 1, 1), (2, 0, 0)]));
        }

        None
    }

    fn check_line_for_state(&self, state: &[[[CellState; 3]; 3]; 3], positions: [(usize, usize, usize); 3]) -> bool {
        let cells = [
            state[positions[0].0][positions[0].1][positions[0].2],
            state[positions[1].0][positions[1].1][positions[1].2],
            state[positions[2].0][positions[2].1][positions[2].2],
        ];

        cells[0] != CellState::Empty && cells[0] == cells[1] && cells[1] == cells[2]
    }

    fn get_winner_from_line_for_state(&self, state: &[[[CellState; 3]; 3]; 3], positions: [(usize, usize, usize); 3]) -> Player {
        let cell = state[positions[0].0][positions[0].1][positions[0].2];
        match cell {
            CellState::Human => Player::Human,
            CellState::AI => Player::AI,
            CellState::Empty => panic!("Empty cell shouldn't be a winner"),
        }
    }

    fn get_possible_moves_for_state(&self, state: &[[[CellState; 3]; 3]; 3]) -> Vec<(usize, usize, usize)> {
        let mut moves = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if state[x][y][z] == CellState::Empty {
                        moves.push((x, y, z));
                    }
                }
            }
        }
        moves
    }

    pub fn backpropagate(&mut self, winner: Player) {
        self.visits += 1;
        if winner == Player::AI {
            self.wins += 1;
        }
    }
}

pub struct MCTSAi {
    pub simulations: u32,
    pub exploration_param: f64,
}

impl MCTSAi {
    pub fn new() -> Self {
        Self {
            simulations: 1000,
            exploration_param: 1.414, // sqrt(2)
        }
    }

    pub fn get_best_move(&self, game_state: &GameState) -> Option<(usize, usize, usize)> {
        if game_state.game_over {
            return None;
        }

        let empty_positions = game_state.get_empty_positions();
        if empty_positions.is_empty() {
            return None;
        }

        // For simplicity, we'll use a basic evaluation approach
        // In a full MCTS implementation, we'd need to properly handle the tree structure
        let mut best_move = None;
        let mut best_score = f64::NEG_INFINITY;

        for &(x, y, z) in &empty_positions {
            let mut total_score = 0.0;
            
            // Run multiple simulations for this move
            for _ in 0..100 {
                let mut sim_state = game_state.board;
                sim_state[x][y][z] = CellState::AI;
                
                let winner = self.simulate_random_game(sim_state, Player::Human);
                let score = match winner {
                    Player::AI => 1.0,
                    Player::Human => -1.0,
                    _ => 0.0, // This won't happen with our enum but keeping for completeness
                };
                total_score += score;
            }

            let avg_score = total_score / 100.0;
            if avg_score > best_score {
                best_score = avg_score;
                best_move = Some((x, y, z));
            }
        }

        best_move
    }

    fn simulate_random_game(&self, mut state: [[[CellState; 3]; 3]; 3], mut current_player: Player) -> Player {
        let mut rng = rand::thread_rng();
        
        loop {
            if let Some(winner) = self.check_winner_for_state(&state) {
                return winner;
            }

            let moves = self.get_possible_moves_for_state(&state);
            if moves.is_empty() {
                // Draw - return random player for simplicity
                return if rng.gen_bool(0.5) { Player::Human } else { Player::AI };
            }

            let (x, y, z) = moves[rng.gen_range(0..moves.len())];
            match current_player {
                Player::Human => state[x][y][z] = CellState::Human,
                Player::AI => state[x][y][z] = CellState::AI,
            }

            current_player = match current_player {
                Player::Human => Player::AI,
                Player::AI => Player::Human,
            };
        }
    }
} 