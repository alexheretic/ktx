use std::{fs::File, io::BufReader};

// #9 panic due to buggy? file texture indices.
#[test]
fn issue9_panic_reading_uffizi_textures() {
    let texture = ktx::include_ktx!("uffizi_rgba16f_cube.ktx");

    dbg!(texture);

    for data in texture.textures() {
        dbg!(data.len());
    }
}

// #9 no panic in runtime read version, but worth keeping for regression testing.
#[test]
fn issue9_panic_reading_uffizi_textures_runtime() {
    let ktx_file = BufReader::new(File::open("tests/uffizi_rgba16f_cube.ktx").unwrap());
    let texture = ktx::Decoder::new(ktx_file).unwrap();

    dbg!(texture.header());

    for data in texture.read_textures() {
        dbg!(data.len());
    }
}
