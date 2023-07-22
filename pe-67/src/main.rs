use std::io::BufRead;

use iter_tools::Itertools;
use pe_utils::get_input_file;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let input = get_input_file(PACKAGE_NAME);

    let mut tree = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    for (row, line) in tree.clone().iter().enumerate().rev().skip(1) {
        for (col, _num) in line.iter().enumerate() {
            let max_below = tree.get_max_below(row, col).unwrap();
            tree[row][col] += max_below;
        }
    }

    println!("{}", tree[0][0]);
}

trait Tree<T>
where
    T: Ord + Copy,
{
    fn get_below(&self, row: usize, col: usize) -> Option<(T, T)>;
    fn get_max_below(&self, row: usize, col: usize) -> Option<T>;
}

impl<T> Tree<T> for Vec<Vec<T>>
where
    T: Ord + Copy,
{
    fn get_below(&self, row: usize, col: usize) -> Option<(T, T)> {
        if row < self.len() - 1 {
            Some((self[row + 1][col], self[row + 1][col + 1]))
        } else {
            None
        }
    }

    fn get_max_below(&self, row: usize, col: usize) -> Option<T> {
        self.get_below(row, col).map(|(a, b)| a.max(b))
    }
}
