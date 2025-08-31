use ddzlib::Tileset;
use std::fs::File;
use std::io::Write;

#[test]
fn parses_12062() {
    let path = "tests/maps/12062/K_12062.idat";
    let mut parsed = Tileset::from_file(path).unwrap();
    println!("{:?}", parsed);
    let data = parsed.read_tile(1, 1, 1).unwrap();
    std::fs::create_dir_all("tests/tmp").unwrap();
    let mut output = File::create("tests/tmp/12062_tile.webp").unwrap();
    output.write_all(data.as_slice()).unwrap();
}
