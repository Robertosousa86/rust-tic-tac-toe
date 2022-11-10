#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Player {
    _O,
    _X,
}

struct Board {
    grid: [[Option<Player>; 3]; 3],
    current_turn: Player,
    winner: Option<Player>,
}

fn main() {
    println!("Hello, world!")
}
