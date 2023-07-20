use pe_utils::get_input_file;

static PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let input = get_input_file(PACKAGE_NAME);
}
