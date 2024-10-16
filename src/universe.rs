//! The `Universe` of the Game of Life. It is a representation of the 2D grid of cells that make up
//! the game.

use std::fmt::{Display, Formatter};

/// A struct that represents the 2D grid of cells that is the universe in the Game of Life.
#[derive(Clone)]
#[allow(dead_code)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Universe {
    /// Creates a new `Universe` of size `width`, `height`.
    pub fn new(width: usize, height: usize) -> Universe {
        Universe {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    /// Get a cell at [x, y] of the `Universe`.
    pub fn get(&self, x: isize, y: isize) -> Option<&bool> {
        if x < 0 || y < 0 {
            return None;
        }

        let cell_index = (y as usize * self.width) + (x as usize);

        self.cells.get(cell_index)
    }

    /// Set a cell at [x, y] of the `Universe`.
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        *self.cells.get_mut(y * self.width + x).unwrap() = value;
    }

    /// Counts the number of neighbors the cell at [x, y] has.
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut neighbors = [false; 8];

        let mut count = 0;

        for i in [-1, 0, 1] {
            let neighbor_x = (x as isize) + i;

            for j in [-1, 0, 1] {

                if i == 0 && j == 0 {
                    continue;
                }

                let neighbor_y = (y as isize) + j;

                neighbors[count] = match self.get(neighbor_x, neighbor_y) {
                    Some(neighbor) => neighbor.clone(),
                    None => false,
                };

                count += 1;
            }
        }
        neighbors.into_iter().filter(|&x| { x }).count() as u8
    }

    /// Returns a `UniverseIterator` to iterate over the cells of a `Universe`.
    pub fn iter(&self) -> UniverseIterator {
        UniverseIterator {
            universe: self,
            index: 0,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let universe_string = self.cells
            .iter()
            .enumerate()
            .map(|(x, value)| {
                format_cell(value, x, self.width)
            }).collect::<String>();

        write!(f, "{}", universe_string)
    }
}

fn format_cell(cell: &bool, i: usize, width: usize) -> String {
    let is_end_of_row = (i + 1) % width == 0;

    let mut cell_string = match cell {
        true => String::from("X"),
        false => String::from(" "),
    };

    if is_end_of_row {
        cell_string += "\n";
    }

    cell_string
}

/// An iterator for a `Universe`.
///
/// It returns Items of (x, y, &bool) where:
/// - x: The x index of the cell.
/// - y: The y index of the cell.
/// - &bool: The value of the cell at [x, y]. This can be read as is_cell_alive.
pub struct UniverseIterator<'a> {
    universe: &'a Universe,
    index: usize,
}

impl<'a> Iterator for UniverseIterator<'a> {
    type Item = (usize, usize, &'a bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.universe.cells.len() {

            let x = self.index % self.universe.width;
            let y = self.index / self.universe.width;

            self.index += 1;

            Some((x, y, &self.universe.get(x as isize, y as isize).unwrap()))
        } else {
            self.index += 1;
            None
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors_1() {
        let mut universe = Universe::new(5, 5);

        universe.set(3, 3, true);

        assert_eq!(universe.count_neighbors(3, 3), 0);
    }

    #[test]
    fn test_count_neighbors_2() {
        let mut universe = Universe::new(5, 5);

        universe.set(3, 3, true);
        universe.set(2, 3, true);
        universe.set(4, 3, true);
        universe.set(3, 4, true);
        universe.set(2, 4, true);
        universe.set(4, 4, true);
        universe.set(3, 2, true);
        universe.set(2, 2, true);
        universe.set(4, 2, true);

        assert_eq!(universe.count_neighbors(3, 3), 8);
        assert_eq!(universe.count_neighbors(4, 4), 3);
    }

    #[test]
    fn test_count_neighbors_3() {
        let mut universe = Universe::new(5, 5);

        universe.set(0, 0, true);
        universe.set(1, 0, true);
        universe.set(1, 1, true);

        assert_eq!(universe.count_neighbors(0, 0), 2);
    }

    #[test]
    fn test_count_neighbors_4() {
        let mut universe = Universe::new(5, 5);

        universe.set(4, 0, true);
        universe.set(3, 0, true);
        universe.set(3, 1, true);
        universe.set(4, 1, true);

        assert_eq!(universe.count_neighbors(4, 0), 3);
    }

    #[test]
    fn test_universe_iterator_1() {
        let universe = Universe::new(5, 5);

        assert_eq!(universe.iter().count(), universe.cells.len());
    }
}
