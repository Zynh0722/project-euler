use std::io::Read;

use pe_utils::get_input_file;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let mut input = get_input_file(PACKAGE_NAME);
    let mut input_string = String::new();
    input.read_to_string(&mut input_string).unwrap();
    let input = input_string;

    println!("{input}");
}
