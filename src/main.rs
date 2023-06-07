// use hexgame_rs::hexgame::game::Game;

use sigma_learning_rs::montecarlo_ts::tree_expansion::*;
fn main() {

    // let state: DecreasingNumbers= DecreasingNumbers{
    //     threshold: 0, numbers: vec![1,2,3,4]
    // };
    // let data: DecreasingNumbersData= DecreasingNumbersData{action:1000};

    // let tree : NodeDN = Tree::expand_decreasing_numbers_tree_rec( state, data);
    // println!("{}",Tree::visit_rec(tree, 0));

    let state: DecreasingNumbers= DecreasingNumbers{
        threshold: 0, numbers: vec![1,2,3,4]
    };
    let data: DecreasingNumbersData= DecreasingNumbersData{action:1000};

    let tree = TreeG<DecreasingNumbers>{system: state, root: None};
    tree.expand_tree();
    println!("{}",tree.visit_rec(tree, 0));
}


