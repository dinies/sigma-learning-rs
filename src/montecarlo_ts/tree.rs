use super::evolving_sytem::*;

pub struct NodeG {
    pub data: Data,
    pub children: Vec<Box<NodeG>>,
}
impl NodeG {
    pub fn visit(&self) -> usize {
        self.data.action
    }
}

pub struct TreeG<System: SystemLike<usize>> {
    pub system: System,
    pub root: NodeG,
}

impl<System> TreeG<System>
where
    System: SystemLike<usize> + Clone,
{
    pub fn expand_tree(&mut self) {
        self.root = TreeG::expand_tree_rec(&mut self.system, Data { action: 1001 })
    }
    fn expand_tree_rec(state: &mut System, data: Data) -> NodeG {
        if state.is_finished() {
            return NodeG {
                data,
                children: Vec::new(),
            };
        }

        let mut children: Vec<Box<NodeG>> = Vec::with_capacity(state.get_multiplicity());
        let possible_actions: &[usize] = state.get_possible_actions();
        for action in possible_actions {
            let mut child_state: System = state.clone();
            child_state.evolve(*action);
            children.push(Box::new(TreeG::expand_tree_rec(
                &mut child_state,
                Data { action: *action },
            )));
        }
        NodeG { data, children }
    }
    pub fn visit(&self) -> String {
        TreeG::<System>::visit_rec(&self.root, 0)
    }
    fn visit_rec(node: &NodeG, level: usize) -> String {
        let mut s: String = String::new();
        for _ in 0..level {
            s.push_str("---");
        }
        s.push_str(&node.visit().to_string());
        s.push('\n');
        if node.children.is_empty() {
            s
        } else {
            for child in node.children.iter() {
                s.push_str(&TreeG::<System>::visit_rec(child, level + 1));
            }
            s
        }
    }
}

//Less generic code

// pub struct NodeDN {
//     pub data: Data,
//     pub children: Vec<Box<NodeDN>>,
// }

// pub struct Tree {
//     pub root: NodeDN,
// }
// impl NodeDN {
//     pub fn visit(&self) -> String {
//         self.data.action.to_string()
//     }
// }

// impl Tree {
//     pub fn expand_decreasing_numbers_tree_rec(
//         state: DecreasingNumbers,
//         data: Data,
//     ) -> NodeDN {
//         if state.is_finished() {
//             return NodeDN {
//                 data,
//                 children: Vec::new(),
//             };
//         }

//         let mut children: Vec<Box<NodeDN>> = Vec::with_capacity(state.get_multiplicity());
//         let possible_actions: &[usize] = state.get_possible_actions();
//         for action in possible_actions {
//             let mut child_state: DecreasingNumbers = state.clone();
//             child_state.evolve(*action);
//             children.push(Box::new(Tree::expand_decreasing_numbers_tree_rec(
//                 child_state,
//                 Data { action: *action },
//             )));
//         }
//         NodeDN { data, children }
//     }
//     pub fn visit_rec(node: NodeDN, level: usize) -> String {
//         let mut s: String = String::new();
//         for _ in 0..level {
//             s.push_str("---");
//         }
//         s.push_str(&node.visit());
//         s.push('\n');
//         if node.children.is_empty() {
//             s
//         } else {
//             for child in node.children {
//                 s.push_str(&Tree::visit_rec(*child, level + 1));
//             }
//             s
//         }
//     }
// }
