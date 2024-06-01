use crate::minesweeper::{GameState, Minesweeper, State};

pub struct NGMinesweeper {
    width: usize,
    height: usize,
    bombs_num: usize,
    minesweeper: Option<Minesweeper>
}

impl NGMinesweeper {
    pub fn new(width: usize, height: usize, bombs_num: usize) -> Self {
        Self { width, height, bombs_num, minesweeper: None }
    }

    pub fn open(&mut self, x: usize, y: usize) -> Option<GameState> {
        assert!(x < self.width && y < self.height);
        let p = x + y * self.width;
        if self.minesweeper.is_none() {
            self.generate(p);
        }
        let Some(minesweeper) = &mut self.minesweeper else { unreachable!() };
        minesweeper.open(x, y)
    }
    
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if let Some(minesweeper) = self.minesweeper.as_mut() {
            minesweeper.toggle_flag(x, y);
        }
    }

    pub fn field(&self) -> Vec<Vec<Option<State>>> {
        if self.minesweeper.is_none() { return vec![vec![None; self.width]; self.height] }
        self.minesweeper.as_ref().unwrap().field()
    }

    pub fn all_reveal(&self) -> Option<Vec<Vec<State>>> {
        Some(self.minesweeper.as_ref()?.all_reveal())
    }

    fn generate(&mut self, start: usize) {
        loop {
            let mut minesweeper = Minesweeper::new_random(self.width, self.height, self.bombs_num);
            if let Some(GameState::Lose) = minesweeper.open(start % self.width, start / self.width) { continue }
            if minesweeper.is_solvable() {
                self.minesweeper = Some(minesweeper);
                break;
            }
        }
    }
}