use rand::Rng;
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
            simulations: 2000, // Increased for better play
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

        // First, check if AI can win immediately
        if let Some(winning_move) = self.find_winning_move(game_state, Player::AI) {
            return Some(winning_move);
        }

        // Second, check if AI needs to block human from winning
        if let Some(blocking_move) = self.find_winning_move(game_state, Player::Human) {
            return Some(blocking_move);
        }

        // Use enhanced MCTS with strategic evaluation
        let mut best_move = None;
        let mut best_score = f64::NEG_INFINITY;

        for &(x, y, z) in &empty_positions {
            let mut total_score = 0.0;
            
            // Run multiple simulations for this move
            let sims_per_move = self.simulations / (empty_positions.len().max(1) as u32);
            for _ in 0..sims_per_move {
                let mut sim_state = game_state.board;
                sim_state[x][y][z] = CellState::AI;
                
                let winner = self.simulate_smart_game(sim_state, Player::Human);
                let score = match winner {
                    Player::AI => 1.0,
                    Player::Human => -1.0,
                };
                total_score += score;
            }

            // Add strategic position evaluation
            let position_value = self.evaluate_position(x, y, z, game_state);
            let avg_score = total_score / sims_per_move as f64;
            let final_score = avg_score + position_value;

            if final_score > best_score {
                best_score = final_score;
                best_move = Some((x, y, z));
            }
        }

        best_move
    }

    // Find if a player can win on their next move
    fn find_winning_move(&self, game_state: &GameState, player: Player) -> Option<(usize, usize, usize)> {
        let empty_positions = game_state.get_empty_positions();
        
        for &(x, y, z) in &empty_positions {
            let mut test_state = game_state.board;
            match player {
                Player::AI => test_state[x][y][z] = CellState::AI,
                Player::Human => test_state[x][y][z] = CellState::Human,
            }
            
            if MCTSAi::check_winner_for_state(&test_state).is_some() {
                return Some((x, y, z));
            }
        }
        
        None
    }

    // Evaluate strategic value of a position
    fn evaluate_position(&self, x: usize, y: usize, z: usize, game_state: &GameState) -> f64 {
        let mut score = 0.0;
        
        // Center positions are more valuable
        let center_distance = ((x as f64 - 1.0).abs() + (y as f64 - 1.0).abs() + (z as f64 - 1.0).abs()) / 3.0;
        score += (1.0 - center_distance) * 0.1;
        
        // Corner positions have strategic value
        if (x == 0 || x == 2) && (y == 0 || y == 2) && (z == 0 || z == 2) {
            score += 0.05;
        }
        
        // Count potential winning lines through this position
        score += self.count_potential_lines(x, y, z, game_state) * 0.02;
        
        score
    }

    // Count how many winning lines pass through this position
    fn count_potential_lines(&self, x: usize, y: usize, z: usize, game_state: &GameState) -> f64 {
        let mut count = 0.0;
        
        // All possible lines through position (x, y, z)
        let lines = [
            // X-axis lines
            [(0, y, z), (1, y, z), (2, y, z)],
            // Y-axis lines  
            [(x, 0, z), (x, 1, z), (x, 2, z)],
            // Z-axis lines
            [(x, y, 0), (x, y, 1), (x, y, 2)],
        ];
        
        // Add diagonal lines if applicable
        let mut diagonal_lines = Vec::new();
        
        // XY plane diagonals
        if x == y {
            diagonal_lines.push([(0, 0, z), (1, 1, z), (2, 2, z)]);
        }
        if x + y == 2 {
            diagonal_lines.push([(0, 2, z), (1, 1, z), (2, 0, z)]);
        }
        
        // XZ plane diagonals
        if x == z {
            diagonal_lines.push([(0, y, 0), (1, y, 1), (2, y, 2)]);
        }
        if x + z == 2 {
            diagonal_lines.push([(0, y, 2), (1, y, 1), (2, y, 0)]);
        }
        
        // YZ plane diagonals
        if y == z {
            diagonal_lines.push([(x, 0, 0), (x, 1, 1), (x, 2, 2)]);
        }
        if y + z == 2 {
            diagonal_lines.push([(x, 0, 2), (x, 1, 1), (x, 2, 0)]);
        }
        
        // 3D space diagonals
        if x == y && y == z {
            diagonal_lines.push([(0, 0, 0), (1, 1, 1), (2, 2, 2)]);
        }
        if x == y && y + z == 2 {
            diagonal_lines.push([(0, 0, 2), (1, 1, 1), (2, 2, 0)]);
        }
        if x + y == 2 && y == z {
            diagonal_lines.push([(0, 2, 0), (1, 1, 1), (2, 0, 2)]);
        }
        if x + y == 2 && y + z == 2 {
            diagonal_lines.push([(0, 2, 2), (1, 1, 1), (2, 0, 0)]);
        }
        
        // Check all lines for potential
        for line in lines.iter().chain(diagonal_lines.iter()) {
            if line.contains(&(x, y, z)) {
                let mut ai_count = 0;
                let mut human_count = 0;
                
                for &(lx, ly, lz) in line {
                    match game_state.board[lx][ly][lz] {
                        CellState::AI => ai_count += 1,
                        CellState::Human => human_count += 1,
                        CellState::Empty => {},
                    }
                }
                
                // Line is valuable if it's not blocked by opponent
                if human_count == 0 {
                    count += 1.0 + ai_count as f64; // More valuable if AI already has pieces in line
                }
            }
        }
        
        count
    }

    fn simulate_random_game(&self, mut state: [[[CellState; 3]; 3]; 3], mut current_player: Player) -> Player {
        let mut rng = rand::thread_rng();
        
        loop {
            if let Some(winner) = MCTSAi::check_winner_for_state(&state) {
                return winner;
            }

            let moves = MCTSAi::get_possible_moves_for_state(&state);
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

    // Simulate game with some strategic intelligence
    fn simulate_smart_game(&self, mut state: [[[CellState; 3]; 3]; 3], mut current_player: Player) -> Player {
        let mut rng = rand::thread_rng();
        
        loop {
            if let Some(winner) = MCTSAi::check_winner_for_state(&state) {
                return winner;
            }

            let moves = MCTSAi::get_possible_moves_for_state(&state);
            if moves.is_empty() {
                // Draw - return random player for simplicity
                return if rng.gen_bool(0.5) { Player::Human } else { Player::AI };
            }

            // Try to make smarter moves during simulation
            let chosen_move = if rng.gen_bool(0.7) { // 70% chance for smart move
                self.choose_smart_move(&state, current_player, &moves)
            } else {
                // 30% chance for random move to add variety
                moves[rng.gen_range(0..moves.len())]
            };

            let (x, y, z) = chosen_move;
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

    // Choose a strategic move during simulation
    fn choose_smart_move(&self, state: &[[[CellState; 3]; 3]; 3], player: Player, moves: &[(usize, usize, usize)]) -> (usize, usize, usize) {
        let mut rng = rand::thread_rng();
        
        // First priority: win immediately if possible
        for &(x, y, z) in moves {
            let mut test_state = *state;
            match player {
                Player::AI => test_state[x][y][z] = CellState::AI,
                Player::Human => test_state[x][y][z] = CellState::Human,
            }
            
            if MCTSAi::check_winner_for_state(&test_state).is_some() {
                return (x, y, z);
            }
        }
        
        // Second priority: block opponent from winning
        let opponent = match player {
            Player::AI => Player::Human,
            Player::Human => Player::AI,
        };
        
        for &(x, y, z) in moves {
            let mut test_state = *state;
            match opponent {
                Player::AI => test_state[x][y][z] = CellState::AI,
                Player::Human => test_state[x][y][z] = CellState::Human,
            }
            
            if MCTSAi::check_winner_for_state(&test_state).is_some() {
                return (x, y, z);
            }
        }
        
        // Third priority: prefer center and strategic positions
        let mut scored_moves: Vec<_> = moves.iter().map(|&(x, y, z)| {
            let mut score = 0.0;
            
            // Center preference
            let center_distance = ((x as f64 - 1.0).abs() + (y as f64 - 1.0).abs() + (z as f64 - 1.0).abs()) / 3.0;
            score += (1.0 - center_distance) * 10.0;
            
            // Corner preference
            if (x == 0 || x == 2) && (y == 0 || y == 2) && (z == 0 || z == 2) {
                score += 5.0;
            }
            
            ((x, y, z), score)
        }).collect();
        
        scored_moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Pick from top 3 moves with some randomness
        let top_moves = std::cmp::min(3, scored_moves.len());
        scored_moves[rng.gen_range(0..top_moves)].0
    }

    fn check_winner_for_state(state: &[[[CellState; 3]; 3]; 3]) -> Option<Player> {
        // Check all possible winning lines in 3D
        // Lines along X axis
        for y in 0..3 {
            for z in 0..3 {
                if MCTSAi::check_line_for_state(state, [(0, y, z), (1, y, z), (2, y, z)]) {
                    return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, y, z), (1, y, z), (2, y, z)]));
                }
            }
        }

        // Lines along Y axis
        for x in 0..3 {
            for z in 0..3 {
                if MCTSAi::check_line_for_state(state, [(x, 0, z), (x, 1, z), (x, 2, z)]) {
                    return Some(MCTSAi::get_winner_from_line_for_state(state, [(x, 0, z), (x, 1, z), (x, 2, z)]));
                }
            }
        }

        // Lines along Z axis
        for x in 0..3 {
            for y in 0..3 {
                if MCTSAi::check_line_for_state(state, [(x, y, 0), (x, y, 1), (x, y, 2)]) {
                    return Some(MCTSAi::get_winner_from_line_for_state(state, [(x, y, 0), (x, y, 1), (x, y, 2)]));
                }
            }
        }

        // Face diagonals on XY planes
        for z in 0..3 {
            if MCTSAi::check_line_for_state(state, [(0, 0, z), (1, 1, z), (2, 2, z)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 0, z), (1, 1, z), (2, 2, z)]));
            }
            if MCTSAi::check_line_for_state(state, [(0, 2, z), (1, 1, z), (2, 0, z)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 2, z), (1, 1, z), (2, 0, z)]));
            }
        }

        // Face diagonals on XZ planes
        for y in 0..3 {
            if MCTSAi::check_line_for_state(state, [(0, y, 0), (1, y, 1), (2, y, 2)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, y, 0), (1, y, 1), (2, y, 2)]));
            }
            if MCTSAi::check_line_for_state(state, [(0, y, 2), (1, y, 1), (2, y, 0)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, y, 2), (1, y, 1), (2, y, 0)]));
            }
        }

        // Face diagonals on YZ planes
        for x in 0..3 {
            if MCTSAi::check_line_for_state(state, [(x, 0, 0), (x, 1, 1), (x, 2, 2)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(x, 0, 0), (x, 1, 1), (x, 2, 2)]));
            }
            if MCTSAi::check_line_for_state(state, [(x, 0, 2), (x, 1, 1), (x, 2, 0)]) {
                return Some(MCTSAi::get_winner_from_line_for_state(state, [(x, 0, 2), (x, 1, 1), (x, 2, 0)]));
            }
        }

        // 3D diagonals (corner to corner)
        if MCTSAi::check_line_for_state(state, [(0, 0, 0), (1, 1, 1), (2, 2, 2)]) {
            return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 0, 0), (1, 1, 1), (2, 2, 2)]));
        }
        if MCTSAi::check_line_for_state(state, [(0, 0, 2), (1, 1, 1), (2, 2, 0)]) {
            return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 0, 2), (1, 1, 1), (2, 2, 0)]));
        }
        if MCTSAi::check_line_for_state(state, [(0, 2, 0), (1, 1, 1), (2, 0, 2)]) {
            return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 2, 0), (1, 1, 1), (2, 0, 2)]));
        }
        if MCTSAi::check_line_for_state(state, [(0, 2, 2), (1, 1, 1), (2, 0, 0)]) {
            return Some(MCTSAi::get_winner_from_line_for_state(state, [(0, 2, 2), (1, 1, 1), (2, 0, 0)]));
        }

        None
    }

    fn check_line_for_state(state: &[[[CellState; 3]; 3]; 3], positions: [(usize, usize, usize); 3]) -> bool {
        let cells = [
            state[positions[0].0][positions[0].1][positions[0].2],
            state[positions[1].0][positions[1].1][positions[1].2],
            state[positions[2].0][positions[2].1][positions[2].2],
        ];

        cells[0] != CellState::Empty && cells[0] == cells[1] && cells[1] == cells[2]
    }

    fn get_winner_from_line_for_state(state: &[[[CellState; 3]; 3]; 3], positions: [(usize, usize, usize); 3]) -> Player {
        let cell = state[positions[0].0][positions[0].1][positions[0].2];
        match cell {
            CellState::Human => Player::Human,
            CellState::AI => Player::AI,
            CellState::Empty => panic!("Empty cell shouldn't be a winner"),
        }
    }

    fn get_possible_moves_for_state(state: &[[[CellState; 3]; 3]; 3]) -> Vec<(usize, usize, usize)> {
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
} 