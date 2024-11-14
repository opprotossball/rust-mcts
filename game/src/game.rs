pub trait Game: Clone {
    fn is_over(&self) -> bool;
    fn player_count(&self) -> usize;
    // returns active player id if current step is not random
    fn active_player(&self) -> Option<usize>;
    // returns score if game is over
    fn scores(&self) -> Option<Vec<f32>>;
    fn is_random_step(&self) -> bool;
    // returns number of possible actions
    fn cache_actions(&mut self) -> usize;
    // returns all possible random states if current step is random
    fn possible_random(&self) -> Option<Vec<Self>>;
    fn make_random_step(&mut self);
    fn make_action(&mut self, action: usize);
}


