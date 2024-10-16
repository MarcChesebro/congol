This is a crate running Conway's Game of Life.

# Conway's Game of Life Explanation
From wikipedia: [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

Conway's Game of Life or simply Life, is a cellular automaton devised by the British
mathematician John Horton Conway in 1970. Evolution is determined by its initial state,
requiring no further input. One interacts with the Game of Life by creating an initial
configuration and observing how it evolves. It is Turing complete and can simulate a
universal constructor or any other Turing machine.

# The Rules
The universe in the Game of Life is a two-dimensional grid of cells. These cells can be one of
two states, live or dead. The game starts with a set state of the grid(universe). Then to
calculate the next step in time you go through every cell in the universe, and using four simple
rules determine the next state of that cell.

1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2. Any live cell with two or three live neighbours lives on to the next generation.
3. Any live cell with more than three live neighbours dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

Following these four rules makes each generation of cells into a pure function of the preceding
one.

# Example
```Rust
use congol::Game;
use std::thread::sleep;
use std::time::Duration;

// Create a new game of life.
let mut game = Game::new(25, 25);

// Seed the first generation.
game.universe.set(12,12,true);
game.universe.set(12,13,true);
game.universe.set(12,14,true);
game.universe.set(13,14,true);
game.universe.set(11,13,true);

// Run 25 generations.
for generation in 0..25 {
    println!("generation: {generation}");
    println!("{game}");
    sleep(Duration::new(0, 100_000_000));
    // This is what updates the universe to the next generation using the 4 rules.
    game.next_generation()
}
```