use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    squares: HashSet<usize>,
    bombs: usize
}

impl Hint {
    pub fn new(squares: HashSet<usize>, bombs: usize) -> Self {
        Self { squares, bombs }
    }

    pub fn squares(&self) -> &HashSet<usize> {
        &self.squares
    }

    pub fn bombs(&self) -> usize {
        self.bombs
    }

    pub fn clear(&mut self) {
        self.squares.clear();
        self.bombs = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.squares.is_empty()
    }

    pub fn difference(&self, other: &Self) -> Self {
        Hint::new(self.squares.difference(&other.squares).copied().collect(), self.bombs - other.bombs)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let intersect = self.squares.intersection(&other.squares).copied().collect::<HashSet<_>>();
        let intersect_len = intersect.len();
        let candidate = Hint::new(intersect, min(intersect_len, min(self.bombs, other.bombs)));
        if max(self.bombs.saturating_sub(self.squares.len() - candidate.squares.len()), other.bombs.saturating_sub(other.squares.len() - candidate.squares.len())) == candidate.bombs {
            Some(candidate)
        } else {
            None
        }
    }
}

impl PartialOrd for Hint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.squares.is_subset(&other.squares) {
            Some(Ordering::Less)
        } else if self.squares.is_superset(&other.squares) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}