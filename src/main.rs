// use hexgame_rs::hexgame::game::Game;

use sigma_learning_rs::montecarlo_ts::evolving_sytem::*;
use sigma_learning_rs::montecarlo_ts::tree::*;

fn main() {
    let state: DecreasingNumbers = DecreasingNumbers {
        threshold: 0,
        numbers: vec![1, 2, 3, 4],
    };

    let mut tree = Tree {
        system: state,
        root: None,
    };
    tree.expand_tree();
    println!("{}", tree.visit());

    let state: IncreasingNumbers = IncreasingNumbers {
        threshold: 10,
        numbers: vec![3, 12, 8, 7],
    };

    let mut tree = Tree {
        system: state,
        root: None,
    };
    tree.expand_tree();
    println!("{}", tree.visit());
}
