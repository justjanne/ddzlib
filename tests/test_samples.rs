use ddzlib::Tileset;

fn test_file(path: &str) -> Tileset {
    let parsed = Tileset::from_file(path).unwrap();
    println!("{:?}", parsed);
    parsed
}

#[test]
fn parses_12062() {
    test_file("tests/maps/12062/K_12062.idat");
}