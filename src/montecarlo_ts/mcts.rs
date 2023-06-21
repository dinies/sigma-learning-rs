use super::evolving_sytem::SystemLike;

pub struct MontecarloNode<D>{
    pub data:D,
    pub children: Vec<Box<MontecarloNode<D>>>,
}

impl<D> MontecarloNode<D>{
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
impl MontecarloData{
    fn new()-> Self{
        Self { n: 0, w: 0.0, p: 0.0 , action: usize::MAX}
    }
}

pub struct MonteCarloTreeSearch{
    root: MontecarloNode<MontecarloData>,
}

impl MonteCarloTreeSearch{
    pub fn execute_search(&self, system: impl SystemLike<usize>){
        self.root = self.execute_search_rec( system );

    }
    fn execute_search_rec( self, system: impl SystemLike<usize>) -> MontecarloNode<MontecarloData> {
        let node = MontecarloNode<MontecarloData>::new();
        let actions = system.getpossibleactions();
        let action = usize::MAX; //policy.chooseAction( actions )
        let new_system = system.evolve( action);
        let child = self.execute_search_rec( new_system );
        node.children.append(child);
        // w_0, n_0, p_0
        // for child in node.children:
        //      update w_0, n_0, p_0

        // node.data
        node
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


