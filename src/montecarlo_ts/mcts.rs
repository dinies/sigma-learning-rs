use super::evolving_sytem::SystemLike;

pub trait Constructible {
    fn new() -> Self;
}

pub struct MontecarloNode<D: Constructible>{
    pub data:D,
    pub children: Vec<Box<MontecarloNode<D>>>,
}

impl<D: Constructible> MontecarloNode<D>{
    pub fn new() -> Self {
        Self { data: D::new(), children: Vec::new() }
    }
}

pub struct MontecarloData{
    n: isize,
    w: f64,
    p: f64,
    action: usize
}
impl Constructible for MontecarloData{
    fn new()-> Self{
        Self { n: 0, w: 0.0, p: 0.0 , action: usize::MAX}
    }
}

pub struct MonteCarloTreeSearch{
    root: MontecarloNode<MontecarloData>,
}

impl MonteCarloTreeSearch{
    pub fn execute_search(&self, system: impl SystemLike<usize> + Clone){
        self.execute_search_rec( &mut self.root,system );

    }
    fn execute_search_rec( self,  node:&mut MontecarloNode<MontecarloData>, system: impl SystemLike<usize> + Clone) {
        let actions = system.get_possible_actions();
        let action = usize::MAX; //policy.chooseAction( actions )
        let mut new_system = system.clone();
        //if this action has already been chosen, continue the recursion on the corresponding child
        //let child = find if action is same in children//
        //else create a new child and recur on it
        let child = MontecarloNode::<MontecarloData>::new();


        node.children.append(Box::new(&mut child));
        new_system.evolve( action);
        self.execute_search_rec( &mut child ,new_system );
        // w_0, n_0, p_0
        // for child in node.children:
        //      update w_0, n_0, p_0

        // node.data
    }
}






#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let x = 6;
        assert!(x == 6);
    }
}


