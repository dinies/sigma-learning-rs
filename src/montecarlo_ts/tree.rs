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
    pub root: Option<Node>,
}

impl<System> Tree<System>
where
    System: SystemLike<usize> + Clone,
{
    pub fn expand_tree(&mut self) {
        self.root = Some(Tree::expand_tree_rec(
            &mut self.system,
            Data { action: usize::MAX },
        ))
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
        match &self.root {
            Some(node) => Tree::<System>::visit_rec(node, 0),
            None => String::from("Empty tree"),
        }
    }
    fn visit_rec(node: &Node, level: usize) -> String {
        let mut s: String = String::new();
        for _ in 0..level {
            s.push_str("---");
        }
        let num = node.visit().to_string();
        if num != usize::MAX.to_string() {
            s.push_str(&num);
        }
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
