use crate::game;
use crate::{game::Game, node};
use crate::node::MctsNode;
use rand::{self, Rng};
use core::{borrow, f32};
use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

pub struct Mcts<T: Game> {
    pub max_simulations: usize,
    pub root: Rc<RefCell<MctsNode<T>>>,
}

impl<T: Game> Mcts<T> {
    pub fn new(max_simulations: usize, state: T) -> Self {
        Mcts {
            max_simulations,
            root: MctsNode::new_rc(None, state)
        }
    }

    pub fn select_action(&mut self) -> usize {
        for _ in 0..self.max_simulations {
            let node: Rc<RefCell<MctsNode<T>>> = self.select_node(Rc::clone(&self.root));
            let scores = if node.borrow().state.is_over() {
                node.borrow().state.scores().unwrap()
            } else {
                MctsNode::expand(&node);
                let new_node = Rc::clone(&node.borrow().children[0]);
                Self::simulate(&new_node)
            };
            MctsNode::backpropagate(node, scores);
        }
        self.root.borrow().children
            .iter()
            .enumerate()
            .max_by_key(|&(_, v)| v.borrow().simulations)
            .map(&|(i, _)| i)
            .unwrap()
    }

    pub fn select_node(&self, node: Rc<RefCell<MctsNode<T>>>) -> Rc<RefCell<MctsNode<T>>> {
        if node.borrow().state.is_random_step() {
            todo!()
        }
        if node.borrow().state.is_over() || node.borrow().children.len() == 0 {
            return node;
        }
        let mut best = 0;
        let mut best_score = f32::MIN;
        for i in 0..node.borrow().children.len() {
            let score = Mcts::selection_policy(&node.borrow().children[i]);
            if score > best_score {
                best_score = score;
                best = i;
            }
        }
        self.select_node(Rc::clone(&node.borrow_mut().children[best]))
    }
    
    fn selection_policy(node: &Rc<RefCell<MctsNode<T>>>) -> f32 {
        // always explore nodes not visited before
        if node.borrow().simulations == 0 {
            return f32::MAX;
        }
        let (parent_simulations, parent_player) = match &node.borrow().parent {
            Some(parent) => (
                parent.upgrade().unwrap().borrow().simulations, 
                parent.upgrade().unwrap().borrow().state.active_player().unwrap()
            ), 
            None => return 0.0
        };
        node.borrow().scores[parent_player] / node.borrow().simulations as f32 +
        f32::sqrt(2.0) * f32::sqrt(f32::ln(parent_simulations as f32) / node.borrow().simulations as f32)
    }

    fn simulate(node: &Rc<RefCell<MctsNode<T>>>) -> Vec<f32>  {
        let mut game = node.borrow().state.clone();
        while !game.is_over() {
            while game.is_random_step() { 
                game.make_random_step(); 
            }
            let action_count = game.cache_actions();
            game.make_action(rand::thread_rng().gen_range(0..action_count));
        }
        game.scores().unwrap().clone()
    }

}
