mod tic_tac_toe;

use mcts::Mcts;
use game::Game;

use crate::tic_tac_toe::TicTacToe;

fn main() {
    let simulations = 124;
    println!("MCTS playing for both sides, {} simulations per move\n", simulations);
    let mut game = TicTacToe::new();
    while !game.is_over() {
        let mut mcts = Mcts::new(simulations, game.clone());
        let action = mcts.select_action();
        game.cache_actions();
        println!("Player {} played on tile {}", game.active_player, game.actions[action]);
        game.make_action(action);
    }
    println!();
    let score= game.scores().unwrap();
    if score[0] == score[1] {
        println!("draw")
    } else if score[0] > score[1] {
        println!("player 0 won")
    } else {
        println!("player 1 won")
    }
}
