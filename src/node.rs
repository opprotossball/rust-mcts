use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};
use crate::game::Game;

pub struct MctsNode<T: Game> {
    pub scores: Vec<f32>,
    pub simulations: usize,
    pub state: T,
    pub parent: Option<Weak<RefCell<MctsNode<T>>>>,
    pub children: Vec<Rc<RefCell<MctsNode<T>>>>
}

impl <T: Game> MctsNode<T> {
    pub fn new_rc(parent: Option<Weak<RefCell<MctsNode<T>>>>, state: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(parent, state)))
    }

    pub fn new(parent: Option<Weak<RefCell<MctsNode<T>>>>, state: T) -> Self {
        MctsNode {
            scores: vec![0.; state.player_count()],
            simulations: 0,
            state: state,
            parent: parent,
            children: vec![]
        }
    }

    pub fn backpropagate(node: Rc<RefCell<MctsNode<T>>>, scores: Vec<f32>) {
        node.borrow_mut().simulations += 1;
        for (i, v) in scores.iter().enumerate() {
            node.borrow_mut().scores[i] += v;
        }
        if let Some(parent_weak) = node.borrow().parent.as_ref() {
            if let Some(parent) = parent_weak.upgrade() {
                MctsNode::backpropagate(parent, scores);
            }
        }
    }

    pub fn expand(node: &Rc<RefCell<MctsNode<T>>>) {
        if node.borrow().state.is_random_step() {
            MctsNode::expand_random(node);
        } else {
            MctsNode::expand_actions(node);
        }
    }

    fn expand_random(node: &Rc<RefCell<MctsNode<T>>>) {
        for new_state in node.borrow().state.possible_random().unwrap() {
            let new_node = MctsNode::new(Some(Rc::downgrade(node)), new_state);
            node.borrow_mut().children.push(Rc::new(RefCell::new(new_node)));
        }
    }

    // adds child node for each possible move
    fn expand_actions(node: &Rc<RefCell<MctsNode<T>>>) {
        let action_count = node.borrow_mut().state.cache_actions();
        for action_id in 0..action_count {
            let mut new_state = node.borrow().state.clone();
            new_state.make_action(action_id);
            let new_node = MctsNode::new(Some(Rc::downgrade(node)), new_state);
            node.borrow_mut().children.push(Rc::new(RefCell::new(new_node)));
        }
    }

    pub fn print(node: &Rc<RefCell<MctsNode<T>>>) {
        println!("simulations: {} | scores: {:?}", node.borrow().simulations, node.borrow().scores);
    }

    pub fn print_tree(node: &Rc<RefCell<MctsNode<T>>>, depth: usize) {
        let indent = " ".repeat(depth);
        let borrowed_node = node.borrow();
        println!("{}{} children", indent, borrowed_node.children.len());
        for child in &borrowed_node.children {
            Self::print_tree(child, depth + 1);
        }
    }
}