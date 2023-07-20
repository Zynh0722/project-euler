use std::collections::HashSet;
use std::io::BufRead;

use iter_tools::Itertools;
use pe_utils::get_input_file;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let input = get_input_file(PACKAGE_NAME);
    let board = input
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.split(" ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let size = board.iter().len();

    let mut visited = HashSet::new();
    let mut max = 0u32;

    for p in (0..400).map(|p| get_pos_2d(p, size)) {
        for pairing in p.get_pairings() {
            if !visited.contains(&pairing) {
                let product: u32 = pairing.iter().map(|Point(x, y)| board[*x][*y]).product();

                if product > max {
                    max = product;
                }

                visited.insert(pairing);
            }
        }
    }

    println!("{max}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);
#[derive(Clone, Copy, Debug)]
struct Direction(isize, isize);

static DIRECTIONS: [Direction; 8] = [
    Direction(1, 0),
    Direction(1, 1),
    Direction(0, 1),
    Direction(-1, 1),
    Direction(-1, 0),
    Direction(-1, -1),
    Direction(0, -1),
    Direction(1, -1),
];

impl Point {
    fn get_pairings(&self) -> Vec<[Point; 4]> {
        DIRECTIONS
            .clone()
            .into_iter()
            .map(|d| self.get_pair(d))
            .flatten()
            .collect_vec()
    }

    fn get_pair(&self, d @ Direction(dx, dy): Direction) -> Option<[Point; 4]> {
        // This check is what allows us to add isizes to usizes
        match self.is_valid_direction(d) {
            true => Some([
                self.clone(),
                Point(
                    (self.0 as isize + dx) as usize,
                    (self.1 as isize + dy) as usize,
                ),
                Point(
                    (self.0 as isize + dx * 2) as usize,
                    (self.1 as isize + dy * 2) as usize,
                ),
                Point(
                    (self.0 as isize + dx * 3) as usize,
                    (self.1 as isize + dy * 3) as usize,
                ),
            ]),
            false => None,
        }
    }

    fn is_valid_direction(&self, Direction(dx, dy): Direction) -> bool {
        !(self.0 < 3 && dx < 0
            || self.0 > 16 && dx > 0
            || self.1 < 3 && dy < 0
            || self.1 > 16 && dy > 0)
    }
}

fn get_pos_2d(pos: usize, grid_size: usize) -> Point {
    Point(pos / grid_size, pos % grid_size)
}
