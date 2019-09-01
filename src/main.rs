use crate::state::State;

mod state;
mod constants;

fn main() {
    println!("Hello, world!");
    let a: State = State { ..Default::default() };
}
