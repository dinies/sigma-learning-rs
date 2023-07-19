// use hexgame_rs::hexgame::game::Game;

use sigma_learning_rs::montecarlo_ts::evolving_sytem::*;
use sigma_learning_rs::montecarlo_ts::mcts::{MonteCarloTreeSearch, MontecarloData, MontecarloNode};
use sigma_learning_rs::montecarlo_ts::tree::*;

fn main() {
    println!("Tree expansion");
    let state= DecreasingNumbers {
        threshold: 0,
        numbers: vec![1, 2, 3, 4],
    };

    let mut tree = Tree {
        system: state,
        root: None,
    };
    tree.expand_tree();
    println!("Decreasing numbers \n{}", tree.visit());

    let state = IncreasingNumbers {
        threshold: 10,
        numbers: vec![3, 12, 8, 7],
    };

    let mut tree = Tree {
        system: state,
        root: None,
    };
    tree.expand_tree();
    println!("Increasing Numbers{}", tree.visit());

    println!("MCTS");
    let data = MontecarloNode::default();
    let mut mcts = MonteCarloTreeSearch{
        root: Box::new(data),
    };


    let system = IncreasingNumbers {
        threshold: 10,
        numbers: vec![3, 12, 8, 7],
    };

    mcts.execute_search(system)

}
