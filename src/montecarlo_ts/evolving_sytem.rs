// use hexgame_rs::hexgame::game::Game;

pub trait SystemLike<Action> {
    fn get_possible_actions(&self) -> &[Action];
    fn evolve(&mut self, action: Action);
    fn is_finished(&self) -> bool;
    fn get_multiplicity(&self) -> usize;
}

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

// pub enum Systems {
//     DecreasingNumbers(DecreasingNumbers),
//     IncreasingNumbers(IncreasingNumbers),
// }
