// use hexgame_rs::hexgame::game::Game;

use sigma_learning_rs::montecarlo_ts::evolving_sytem::*;
use sigma_learning_rs::montecarlo_ts::tree::*;

fn main() {
    let state: DecreasingNumbers = DecreasingNumbers {
        threshold: 0,
        numbers: vec![1, 2, 3, 4],
    };
    let data: Data = Data { action: 1000 };
    let initial_node: Node = Node {
        data,
        children: Vec::new(),
    };

    let mut tree = Tree {
        system: state,
        root: initial_node,
    };
    tree.expand_tree();
    println!("{}", tree.visit());

    let state: IncreasingNumbers = IncreasingNumbers {
        threshold: 10,
        numbers: vec![3, 1, 4, 12, 8, 7],
    };
    let data: Data = Data { action: 3000 };
    let initial_node: Node = Node {
        data,
        children: Vec::new(),
    };

    let mut tree = Tree {
        system: state,
        root: initial_node,
    };
    tree.expand_tree();
    println!("{}", tree.visit());

    //DEPRECATED less generic case
    // let state: DecreasingNumbers= DecreasingNumbers{
    //     threshold: 0, numbers: vec![1,2,3,4]
    // };
    // let data: Data= Data{action:1000};

    // let tree : NodeDN = Tree::expand_decreasing_numbers_tree_rec( state, data);
    // println!("{}",Tree::visit_rec(tree, 0));
}
