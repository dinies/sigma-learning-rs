use hexgame_rs::hexgame::game::Game;

//TODO: GENERIC IMPLEMENTATION

// pub struct Tree<S,D>{
//     pub root: Node<D>,
// }

// pub trait ExtractDataFromState {
//     fn

// }
// impl Tree<S,D>{
//     pub fn expand_hexgame_tree_rec(&self, state: S, data: D )-> Node<D>{

//         if state.game_ended() {
//             Node<D>( last_action, Vec::new() )
//         }

//         let mut children: Vec<Node<T> = Vec::with_capacity(state.get_multiplicity());
//         let possible_actions = state.get_possible_actions();
//         for action in possible_actions {
//             children.push( self.expand_hexgame_tree_rec( state.do( action ), action ));
//         }
//     }
// }

//TODO:  GOOD STARTING POINT
//FIX:add missing funcitons to the API of hexgame_rs

// impl Tree{
//     pub fn expand_hexgame_tree_rec(&self, state: Game, data: HexgameData )-> Node<HexgameData>{

//         if state.game_ended() {
//             Node<HexgameData>( HexgameData(last_action), Vec::new() )
//         }

//         let mut children: Vec<Node<HexgameData> = Vec::with_capacity(state.get_multiplicity());
//         let possible_actions: Vec<(usize, usize)> = state.get_possible_actions();
//         for action in possible_actions {
//             children.push( self.expand_hexgame_tree_rec( state.do_action( action ), action ));
//         }
//         Node<HexgameData>( HexgameData(last_action), children )
//     }
// }

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Box<Node<T>>>,
}

pub struct NodeDN {
    pub data: DecreasingNumbersData,
    pub children: Vec<Box<NodeDN>>,
}

pub struct Tree {
    pub root: NodeDN,
}

impl Tree {
    pub fn expand_decreasing_numbers_tree_rec(
        state: DecreasingNumbers,
        data: DecreasingNumbersData,
    ) -> NodeDN {
        if state.is_finished() {
            return NodeDN {
                data,
                children: Vec::new(),
            };
        }

        let mut children: Vec<Box<NodeDN>> = Vec::with_capacity(state.get_multiplicity());
        let possible_actions: &[usize] = state.get_possible_actions();
        for action in possible_actions {
            children.push(Box::new(Tree::expand_decreasing_numbers_tree_rec(
                state.do_action(*action),
                DecreasingNumbersData { action: *action },
            )));
        }
        NodeDN { data, children }
    }
}

pub struct HexgameData {
    pub action: (usize, usize),
}

pub struct DecreasingNumbersData {
    pub action: usize,
}

pub struct DecreasingNumbers {
    threshold: usize,
    numbers: Vec<usize>, //odered from smallest to biggest
}

impl DecreasingNumbers {
    pub fn new(initial_numbers: Vec<usize>, threshold: usize) -> Self {
        let mut numbers: Vec<usize> = Vec::with_capacity(initial_numbers.len());
        for num in initial_numbers {
            if num > threshold {
                numbers.push(num);
            }
        }
        Self { threshold, numbers }
    }
    pub fn get_possible_actions(&self) -> &[usize] {
        self.numbers.as_slice()
    }
    pub fn do_action(&self, number: usize) -> Self {
        let mut new_numbers = self.numbers.to_owned();
        new_numbers.retain(|&x| x < number);
        Self {
            threshold: self.threshold,
            numbers: new_numbers,
        }
    }
    pub fn is_finished(&self) -> bool {
        self.numbers.is_empty()
    }
    pub fn get_multiplicity(&self) -> usize {
        self.numbers.len()
    }
}
