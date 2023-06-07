// use hexgame_rs::hexgame::game::Game;

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


pub struct NodeDN {
    pub data: DecreasingNumbersData,
    pub children: Vec<Box<NodeDN>>,
}

pub struct Tree {
    pub root: NodeDN,
}
impl NodeDN{
    pub fn visit(&self) -> String {
        String::from(self.data.action.to_string())
    }
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
            let mut child_state :DecreasingNumbers = state.clone();
            child_state.evolve(*action);
            children.push(Box::new(Tree::expand_decreasing_numbers_tree_rec(
                child_state,
                DecreasingNumbersData { action: *action },
            )));
        }
        NodeDN { data, children }
    }
    pub fn visit_rec(node: NodeDN, level: usize) -> String {
        let mut s: String = String::new();
        for _ in 0..level{
            s.push_str("---");
        }
        s.push_str( &node.visit());
        s.push('\n');
        if node.children.is_empty(){
            s
        }
        else{
            for child in node.children{
                s.push_str( &Tree::visit_rec( *child, level+1));
            }
            s
        }
    }
}

pub struct HexgameData {
    pub action: (usize, usize),
}

pub struct DecreasingNumbersData {
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

impl SystemLike for DecreasingNumbers{
    type ActionG = usize;
    fn get_possible_actions(&self) -> &[Self::ActionG]{
        self.numbers.as_slice()
    }
    fn evolve(&mut self, action: Self::ActionG){
        let mut new_numbers = self.numbers.to_owned();
        new_numbers.retain(|&x| x < action);
        self.numbers= new_numbers;
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

impl SystemLike for IncreasingNumbers{
    type ActionG = usize;
    fn get_possible_actions(&self) -> &[Self::ActionG]{
        self.numbers.as_slice()
    }
    fn evolve(&mut self, action: Self::ActionG){
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



pub trait SystemLike{
    type ActionG;
    fn get_possible_actions(&self) -> &[Self::ActionG];
    fn evolve(&mut self, action: Self::ActionG);
    fn is_finished(&self) -> bool;
    fn get_multiplicity(&self) -> usize;
}


pub enum Systems
    {
        DecreasingNumbers( DecreasingNumbers),
        IncreasingNumbers( IncreasingNumbers),
}


pub struct NodeG {
    pub data: DecreasingNumbersData,
    pub children: Vec<Box<NodeG>>,
}
impl NodeG{
    pub fn visit(&self) -> String {
        String::from(self.data.action.to_string())
    }
}

pub struct TreeG<System: SystemLike>{
    system: System,
    root: NodeG,
}


impl<System> TreeG<System>
    where
        System: SystemLike + Clone,
{
    fn expand_tree(&mut self){
        self.root = TreeG::expand_tree_rec( self.system, DecreasingNumbersData{action:1001} )

    }
    fn expand_tree_rec(
        state: System,
        data: DecreasingNumbersData,
    ) -> NodeG {
        if state.is_finished() {
            return NodeG {
                data,
                children: Vec::new(),
            };
        }

        let mut children: Vec<Box<NodeG>> = Vec::with_capacity(state.get_multiplicity());
        let possible_actions: &[System::ActionG] = state.get_possible_actions();
        for action in possible_actions {
            let mut child_state : System = state.clone();
            child_state.evolve(*action);
            children.push(Box::new(TreeG::expand_tree_rec(
                child_state,
                DecreasingNumbersData{ action: *action },
            )));
        }
        NodeG { data, children }
    }
    fn visit(& self) -> String{
        TreeG::<System>::visit_rec(self.root, 0)
    }
    fn visit_rec(node: NodeG, level: usize) -> String {
        let mut s: String = String::new();
        for _ in 0..level{
            s.push_str("---");
        }
        s.push_str( &node.visit());
        s.push('\n');
        if node.children.is_empty(){
            s
        }
        else{
            for child in node.children{
                s.push_str( &TreeG::<System>::visit_rec( *child, level+1));
            }
            s
        }
    }
}




