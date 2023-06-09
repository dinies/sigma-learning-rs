// use hexgame_rs::hexgame::game::Game;

pub struct Data {
    pub action: usize,
}

#[derive(Clone)]
pub struct DecreasingNumbers {
    pub threshold: usize,
    pub numbers: Vec<usize>, //odered from smallest to biggest
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
}

impl SystemLike<usize> for DecreasingNumbers {
    fn get_possible_actions(&self) -> &[usize] {
        self.numbers.as_slice()
    }
    fn evolve(&mut self, action: usize) {
        let mut new_numbers = self.numbers.to_owned();
        new_numbers.retain(|&x| x < action);
        self.numbers = new_numbers;
    }
    fn is_finished(&self) -> bool {
        self.numbers.is_empty()
    }
    fn get_multiplicity(&self) -> usize {
        self.numbers.len()
    }
}

#[derive(Clone)]
pub struct IncreasingNumbers {
    pub threshold: usize,
    pub numbers: Vec<usize>, //odered from smallest to biggest
}

impl IncreasingNumbers {
    pub fn new(initial_numbers: Vec<usize>, threshold: usize) -> Self {
        let mut numbers: Vec<usize> = Vec::with_capacity(initial_numbers.len());
        for num in initial_numbers {
            if num < threshold {
                numbers.push(num);
            }
        }
        Self { threshold, numbers }
    }
}

impl SystemLike<usize> for IncreasingNumbers {
    fn get_possible_actions(&self) -> &[usize] {
        self.numbers.as_slice()
    }
    fn evolve(&mut self, action: usize) {
        let mut new_numbers = self.numbers.to_owned();
        new_numbers.retain(|&x| x > action);
        self.numbers = new_numbers;
    }
    fn is_finished(&self) -> bool {
        self.numbers.is_empty()
    }
    fn get_multiplicity(&self) -> usize {
        self.numbers.len()
    }
}

pub trait SystemLike<Action> {
    fn get_possible_actions(&self) -> &[Action];
    fn evolve(&mut self, action: Action);
    fn is_finished(&self) -> bool;
    fn get_multiplicity(&self) -> usize;
}

pub enum Systems {
    DecreasingNumbers(DecreasingNumbers),
    IncreasingNumbers(IncreasingNumbers),
}

// pub struct HexgameData {
//     pub action: (usize, usize),
// }

// pub trait StateAction<Action> {
//     fn get_possible_actions() -> Vec<Action>;
//     fn evolve(&self, action: Action) -> Self;
// }

// pub enum Systems<T: StateAction<usize>>{
//     DecreasingNumbers(DecreasingNumbers: StateAction),
//     IncreasingNumbers(IncreasingNumbers: StateAction),
// }

// pub struct System{
// }

// #[derive(Debug)]
// pub struct Node<T> {
//     pub data: T,
//     pub children: Vec<Box<Node<T>>>,
// }

// pub trait TreeLike<SystemType, NodeDataType> {
//     fn expand(mut &self, system: SystemType );
//     fn visit(&self ) -> NodeDataType;
// }

// pub struct TreeGeneric<System: StateAction<usize>, NodeData> {
//     root: Option<Node<NodeData>>
// }

// pub struct DecreasingNumbersDataG<Action> {
//     pub action: Action,
// }

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
