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
    n: isize,
    w: f64,
    p: f64,
    action: usize
}
impl Default for MontecarloData{
    fn default()-> Self{
        Self { n: 0, w: 0.0, p: 0.0 , action: usize::MAX}
    }
}

pub struct MonteCarloTreeSearch{
    root: Box<MontecarloNode<MontecarloData>>,
}

impl MonteCarloTreeSearch{
    pub fn execute_search(&mut self, system: impl SystemLike<usize> + Clone){
        MonteCarloTreeSearch::execute_search_rec( &mut self.root ,system );

    }
    fn execute_search_rec(  node:&mut MontecarloNode<MontecarloData>, system: impl SystemLike<usize> + Clone) {
        if system.is_finished() {
            //get final score and use it to update w

        }
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
                MonteCarloTreeSearch::execute_search_rec( &mut child ,new_system );
                node.children.push(child);
            },
        }
        // w_0, n_0, p_0
        // for child in node.children:
        //      update w_0, n_0, p_0

        // node.data
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


