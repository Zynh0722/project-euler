use std::io::BufRead;

use iter_tools::Itertools;
use pe_utils::get_input_file;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let input = get_input_file(PACKAGE_NAME);

    let tree = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    println!("{}", naive_recursive_max(&tree));
}

fn naive_recursive_max(inp: &Vec<Vec<u32>>) -> u32 {
    inp[0][0] + naive_recursive_helper(inp, 1, (0, 1))
}

fn naive_recursive_helper(inp: &Vec<Vec<u32>>, row: usize, (a, b): (usize, usize)) -> u32 {
    if row < inp.len() {
        let a = inp[row]
            .get(a)
            .map(|num| num + naive_recursive_helper(inp, row + 1, (a, a + 1)))
            .unwrap_or(0);

        let b = inp[row]
            .get(b)
            .map(|num| num + naive_recursive_helper(inp, row + 1, (b, b + 1)))
            .unwrap_or(0);

        a.max(b)
    } else {
        0
    }
}
