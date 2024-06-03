use std::collections::{HashMap, HashSet};

use crate::hint::Hint;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResultState {
    Safe,
    Danger
}

#[derive(Default)]
pub struct Solver {
    hints: Vec<Hint>,
    confirmed: HashSet<usize>
}

impl Solver {
    pub fn add_hint(&mut self, new_hint: Hint) {
        let mut new_hints = vec![new_hint];
        'a: while let Some(new_hint) = new_hints.pop() {
            if new_hint.is_empty() { continue }
            if new_hint.squares().len() != 1 {
                if new_hint.squares().len() == new_hint.bombs() {
                    for &square in new_hint.squares() {
                        if !self.confirmed.contains(&square) {
                            new_hints.push(Hint::new(HashSet::from([square]), 1));
                        }
                    }
                    continue;
                }
                if new_hint.bombs() == 0 {
                    for &square in new_hint.squares() {
                        if !self.confirmed.contains(&square) {
                            self.add_hint(Hint::new(HashSet::from([square]), 0));
                        }
                    }
                    continue;
                }
            }
            for hint in self.hints.iter_mut() {
                if hint.is_empty() { continue }
                if *hint == new_hint { continue 'a }
                if *hint < new_hint {
                    new_hints.push(new_hint.difference(hint));
                    continue 'a;
                }
                if let Some(intersection) = hint.intersection(&new_hint) {
                    if !intersection.is_empty() {
                        new_hints.push(hint.difference(&intersection));
                        new_hints.push(new_hint.difference(&intersection));
                        new_hints.push(intersection);
                        hint.clear();
                        continue 'a;
                    }
                }
                if *hint > new_hint {
                    new_hints.push(hint.difference(&new_hint));
                    hint.clear();
                }
            }
            if new_hint.squares().len() == 1 {
                self.confirmed.insert(*new_hint.squares().iter().last().unwrap());
            }
            self.hints.push(new_hint);
            self.compact();
        }
    }

    pub fn result(&self) -> HashMap<usize, ResultState> {
        let mut ans = HashMap::new();
        for hint in &self.hints {
            if hint.squares().len() == 1 {
                let square = *hint.squares().iter().last().unwrap();
                let state = match hint.bombs() {
                    0 => ResultState::Safe,
                    1 => ResultState::Danger,
                    _ => unreachable!()
                };
                ans.insert(square, state);
            }
        }
        ans
    }

    fn compact(&mut self) {
        self.hints = self.hints.iter().filter(|h| !h.is_empty()).cloned().collect();
    }
}