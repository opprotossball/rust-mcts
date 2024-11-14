use core::fmt;

use game::Game;

#[derive(Clone)]
pub struct TicTacToe {
    board: [i8; 9],
    pub active_player: usize,
    winner: Option<usize>,
    pub actions: Vec<TicTacToeMove>,
    moves_made: usize
}

impl TicTacToe {
    fn check_winner(&mut self) {
        for &line in &[
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
            [0, 4, 8], [2, 4, 6],            // diagonals
        ] {
            if self.board[line[0]] != 0 
                && self.board[line[0]] == self.board[line[1]] 
                && self.board[line[0]] == self.board[line[2]] {
                    self.winner = if self.board[line[0]] == -1 {Some(0)} else {Some(1)};
                    return;
            }
        }
        self.winner = None;
    }

    pub fn new() -> Self {
        TicTacToe {
            board: [0; 9],
            active_player: 0,
            winner: None,
            actions: vec![],
            moves_made: 0
        }
    }
}

impl Game for TicTacToe {
    fn is_over(&self) -> bool {
        self.winner.is_some() || self.moves_made == 9
    }

    fn player_count(&self) -> usize {
        2
    }

    fn active_player(&self) -> Option<usize> {
        Some(self.active_player)
    }

    fn scores(&self) -> Option<Vec<f32>> {
        match self.winner {
            None => Some(vec![0., 0.]),
            Some(0) => Some(vec![1., -1.]),
            _ => Some(vec![-1., 1.])
        }
    }

    fn is_random_step(&self) -> bool {
        false
    }

    fn cache_actions(&mut self) -> usize {
        self.actions = self.board
                .iter()
                .enumerate()
                .filter(|&(_, v)| v == &0)
                .map(|(i, _)| TicTacToeMove {tile: i})
                .collect();
        self.actions.len()
    }

    fn possible_random(&self) -> Option<Vec<TicTacToe>> {
        None
    }

    fn make_random_step(&mut self) {

    }

    fn make_action(&mut self, action_id: usize) {
        self.moves_made += 1;
        if self.actions.len() <= action_id {
            panic!("wrong action_id or actions not cached")
        }
        let action = &self.actions[action_id];
        if self.board[action.tile] != 0 {
            panic!("tile already occupied")
        }
        match self.active_player {
           0 => {
            self.board[action.tile] = -1;
            self.active_player = 1;
           }
           _ => {
            self.board[action.tile] = 1;
            self.active_player = 0;
           }
        }
        self.check_winner();
    }
}

#[derive(Clone)]
pub struct TicTacToeMove {
    pub tile: usize
}

impl fmt::Display for TicTacToeMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile)
    }
}
