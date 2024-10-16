//! This is a crate running Conway's Game of Life.
//!
//! To see examples of running a game check out [`Game`](struct.Game.html).
//!
//! # Conway's Game of Life Explanation
//! From wikipedia: [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
//!
//! Conway's Game of Life or simply Life, is a cellular automaton devised by the British
//! mathematician John Horton Conway in 1970. Evolution is determined by its initial state,
//! requiring no further input. One interacts with the Game of Life by creating an initial
//! configuration and observing how it evolves. It is Turing complete and can simulate a universal
//! constructor or any other Turing machine.
//!
//! # The Rules
//! The universe in the Game of Life is a two-dimensional grid of cells. These cells can be one of
//! two states, live or dead. The game starts with a set state of the grid(universe). Then to
//! calculate the next step in time you go through every cell in the universe, and using four simple
//! rules determine the next state of that cell.
//!
//! 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
//! 2. Any live cell with two or three live neighbours lives on to the next generation.
//! 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
//! 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
//!
//! Following these four rules makes each generation of cells into a pure function of the preceding
//! one.

pub mod universe;

use std::fmt::Display;
use universe::Universe;

/// A struct that holds the metadata about the game as well as the universe grid. Use this to run a
/// game of life.
///
/// # Examples
/// ```
/// use congol::Game;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// // Create a new game of life.
/// let mut game = Game::new(25, 25);
///
/// // Seed the first generation.
/// game.universe.set(12,12,true);
/// game.universe.set(12,13,true);
/// game.universe.set(12,14,true);
/// game.universe.set(13,14,true);
/// game.universe.set(11,13,true);
///
/// // Run 25 generations.
/// for generation in 0..25 {
///     println!("generation: {generation}");
///     println!("{game}");
///     sleep(Duration::new(0, 100_000_000));
///
///     // This is what updates the universe to the next generation using the 4 rules.
///     game.next_generation()
/// }
/// ```
pub struct Game {
    pub universe: Universe,
}

impl Game {
    /// Creates a new `Game` with a `Universe` of size `width`, `height`.
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            universe: Universe::new(width, height),
        }
    }

    /// Runs through the entire `self.universe` and applies the four rules to generate the next
    /// generation. This will replace the current `self.universe` with the new one generated.
    pub fn next_generation(&mut self) {
        // copy previous universe
        let previous_universe = self.universe.clone();

        // go through the entire previous universe cell by cell
        for (x, y, cell) in previous_universe.iter() {
            // count the neighbors of the cell
            let cell_neighbor_count = previous_universe.count_neighbors(x, y);

            // calculate the new state based on the amount of neighbors
            let new_state = determine_new_state(cell, cell_neighbor_count);

            // set the current universe to the new state
            self.universe.set(x, y, new_state);
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.universe.to_string())
    }
}

fn determine_new_state(is_cell_alive: &bool, neighbor_count: u8) -> bool {
    if *is_cell_alive {
        match neighbor_count {
            0 | 1 => false, // depopulation
            2 | 3 => true,  // stays alive, balance
            _ => false,     // overcrowding
        }
    } else {
        match neighbor_count {
            3 => true,      // reproduction
            _ => false,     // nothing
        }
    }
}
