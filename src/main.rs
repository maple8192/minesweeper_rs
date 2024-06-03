use proconio::input;

use crate::minesweeper::State;
use crate::ng_minesweeper::NGMinesweeper;

mod solver;
mod hint;
mod minesweeper;
mod ng_minesweeper;

fn main() {
    let mut minesweeper = NGMinesweeper::new(10, 10, 10);
    show(minesweeper.field());
    while {
        input! {
            x: usize,
            y: usize,
            f: usize
        }
        let ret = if f == 0 {
            minesweeper.open(x, y)
        } else {
            minesweeper.toggle_flag(x, y);
            None
        };
        show(minesweeper.field());
        ret.inspect(|x| println!("{:?}", x));
        ret.is_none()
    } {}
    show(minesweeper.all_reveal().unwrap().into_iter().map(|x| x.into_iter().map(Some).collect()).collect());
}

fn show(field: Vec<Vec<Option<State>>>) {
    println!("{}", field.into_iter().map(|r| r.into_iter().map(|x| match x {
        Some(State::Revealed(n)) => n.to_string(),
        Some(State::Flagged) => "F".to_string(),
        None => "-".to_string()
    }).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n"));
}
