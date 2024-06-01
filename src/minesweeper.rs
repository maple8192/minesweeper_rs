use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::hint::Hint;
use crate::solver::{ResultState, Solver};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    Win,
    Lose
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Revealed(usize),
    Flagged
}

#[derive(Debug, Clone)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    bombs: HashSet<usize>,
    states: HashMap<usize, State>
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, bombs: HashSet<usize>) -> Self {
        Self { width, height, bombs, states: HashMap::new() }
    }

    pub fn new_random(width: usize, height: usize, bombs_num: usize) -> Self {
        let mut rand = thread_rng();
        let mut squares = (0..width * height).collect::<Vec<_>>();
        squares.shuffle(&mut rand);
        let bombs = squares.into_iter().take(bombs_num).collect();
        Self::new(width, height, bombs)
    }

    pub fn open(&mut self, x: usize, y: usize) -> Option<GameState> {
        assert!(x < self.width && y < self.height);
        let p = x + y * self.width;
        if self.bombs.contains(&p) {
            return Some(GameState::Lose);
        }
        if self.states.get(&p).is_none() {
            self.states.insert(p, State::Revealed(self.get_bombs_around(x, y)));
        }
        if self.states.iter().filter(|(_, &s)| matches!(s, State::Revealed(_))).collect::<Vec<_>>().len() == self.width * self.height - self.bombs.len() {
            return Some(GameState::Win);
        }
        None
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        assert!(x < self.width && y < self.height);
        let p = x + y * self.width;
        match self.states.get(&p) {
            Some(State::Flagged) => { self.states.remove(&p); }
            None => { self.states.insert(p, State::Flagged); }
            _ => ()
        }
    }

    pub fn field(&self) -> Vec<Vec<Option<State>>> {
        let mut field = vec![vec![None; self.width]; self.height];
        for (&p, &state) in &self.states {
            field[p / self.width][p % self.width] = Some(state);
        }
        field
    }

    pub fn all_reveal(&self) -> Vec<Vec<State>> {
        let mut field = vec![vec![State::Flagged; self.width]; self.height];
        for (y, row) in field.iter_mut().enumerate() {
            for (x, state) in row.iter_mut().enumerate() {
                let p = x + y * self.width;
                if !self.bombs.contains(&p) {
                    *state = State::Revealed(self.get_bombs_around(x, y));
                }
            }
        }
        field
    }

    pub fn is_solvable(&self) -> bool {
        let mut solver = Solver::default();
        let mut revealed = HashSet::new();
        let mut opens = self.states.iter().filter_map(|(&p, &s)| if let State::Revealed(_) = s { Some(p) } else { None }).collect::<Vec<_>>();
        while let Some(open) = opens.pop() {
            revealed.insert(open);
            solver.add_hint(Hint::new(HashSet::from([open]), 0));
            solver.add_hint(Hint::new(self.get_around(open % self.width, open / self.width).into_iter().collect(), self.get_bombs_around(open % self.width, open / self.width)));

            for (p, s) in solver.result() {
                if s == ResultState::Safe && !revealed.contains(&p) {
                    opens.push(p);
                }
            }
        }
        self.width * self.height - revealed.len() == self.bombs.len()
    }
    
    fn get_around(&self, x: usize, y: usize) -> Vec<usize> {
        (-1..=1).flat_map(|x| (-1..=1).map(|y| (x, y)).collect::<Vec<_>>())
            .filter(|x| x != &(0, 0))
            .map(|(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
            .filter(|&(x, y)| x < self.width && y < self.height)
            .map(|(x, y)| x + y * self.width)
            .collect()
    }

    fn get_bombs_around(&self, x: usize, y: usize) -> usize {
        self.get_around(x, y).iter().filter(|p| self.bombs.contains(p)).collect::<Vec<_>>().len()
    }
}