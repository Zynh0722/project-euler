use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn get_input_file(package_name: &str) -> BufReader<File> {
    let path = Path::new("inputs").join(package_name);

    BufReader::new(File::open(path).unwrap())
}
