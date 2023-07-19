use super::evolving_sytem::SystemLike;


pub struct MontecarloNode<D: Default>{
    pub data:D,
    pub children: Vec<Box<MontecarloNode<D>>>,
}

impl<D: Default> Default for MontecarloNode<D>{
    fn default() -> Self {
        Self { data: D::default(), children: Vec::new() }
    }
}

pub struct MontecarloData{
    n: isize, //number of times a node has been visited
    w: f64, //aggregate of the final score of the node and its sub-tree
    p: f64, // I don't know yet
    action: usize
}
impl Default for MontecarloData{
    fn default()-> Self{
        Self { n: 0, w: 0.0, p: 0.0 , action: usize::MAX}
    }
}
impl ToString for MontecarloData{
    fn to_string(&self)-> String{
        format!("Mcts Node\nn: {}\nw: {}\np: {}\naction: {}",self.n,self.w,self.p,self.action)
    }
}

pub struct MonteCarloTreeSearch{
    pub root: Box<MontecarloNode<MontecarloData>>,
}

impl MonteCarloTreeSearch{
    pub fn execute_search(&mut self, system: impl SystemLike<usize> + Clone){
        MonteCarloTreeSearch::execute_search_rec( &mut self.root ,system );

    }
    fn execute_search_rec(  node:&mut MontecarloNode<MontecarloData>, system: impl SystemLike<usize> + Clone) {
        node.data.n += 1;
        if system.is_finished() {
            node.data.w = 1.0;
        } else {
            let actions = system.get_possible_actions();
            let chosen_action = usize::MAX; //policy.chooseAction( actions )
            let mut new_system = system.clone();
            new_system.evolve( chosen_action);
            //if this action has already been chosen, continue the recursion on the corresponding child

            match  node.children.iter_mut().find(|child| child.data.action == chosen_action){
                Some( child) => {
                    MonteCarloTreeSearch::execute_search_rec( child ,new_system );
                },
                None =>{
                    let mut child = Box::<MontecarloNode::<MontecarloData>>::default();
                    child.data.action = chosen_action;
                    MonteCarloTreeSearch::execute_search_rec( &mut child ,new_system );
                    node.children.push(child);
                },
            }

            node.data.w = node.children.iter().map(|child| child.data.w).sum();
            // w_0, n_0, p_0
            // for child in node.children:
            //      update w_0, n_0, p_0

            // node.data
        }
    }
    pub fn visit(&self) -> String {
        MonteCarloTreeSearch::visit_rec(&*self.root)
    }
    fn visit_rec(node: &MontecarloNode<MontecarloData>) -> String {
        let mut s: String = String::new();
        s
        // for _ in 0..level {
        //     s.push_str("---");
        // }
        // let num = node.visit().to_string();
        // if num != usize::MAX.to_string() {
        //     s.push_str(&num);
        // }
        // s.push('\n');
        // if node.children.is_empty() {
        //     s
        // } else {
        //     for child in node.children.iter() {
        //         s.push_str(&MonteCarloTreeSearch::visit_rec(child, level + 1));
        //     }
        //     s
        // }
    }

}

   // let mut child_state: System = state.clone();
   //          child_state.evolve(*action);
   //          children.push(Box::new(Tree::expand_tree_rec(
   //              &mut child_state,
   //              Data { action: *action },
   //          )));
   //      }






#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let x = 6;
        assert!(x == 6);
    }
}


