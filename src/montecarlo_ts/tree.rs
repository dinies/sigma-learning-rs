use super::evolving_sytem::*;

pub struct Node {
    pub data: Data,
    pub children: Vec<Box<Node>>,
}
impl Node {
    pub fn visit(&self) -> usize {
        self.data.action
    }
}

pub struct Tree<System: SystemLike<usize>> {
    pub system: System,
    pub root: Node,
}

impl<System> Tree<System>
where
    System: SystemLike<usize> + Clone,
{
    pub fn expand_tree(&mut self) {
        self.root = Tree::expand_tree_rec(&mut self.system, Data { action: 1001 })
    }
    fn expand_tree_rec(state: &mut System, data: Data) -> Node {
        if state.is_finished() {
            return Node {
                data,
                children: Vec::new(),
            };
        }

        let mut children: Vec<Box<Node>> = Vec::with_capacity(state.get_multiplicity());
        let possible_actions: &[usize] = state.get_possible_actions();
        for action in possible_actions {
            let mut child_state: System = state.clone();
            child_state.evolve(*action);
            children.push(Box::new(Tree::expand_tree_rec(
                &mut child_state,
                Data { action: *action },
            )));
        }
        Node { data, children }
    }
    pub fn visit(&self) -> String {
        Tree::<System>::visit_rec(&self.root, 0)
    }
    fn visit_rec(node: &Node, level: usize) -> String {
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
                s.push_str(&Tree::<System>::visit_rec(child, level + 1));
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
