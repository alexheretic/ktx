use ktx::*;
use std::{fs::File, io::BufReader};

// #9 panic due to buggy? file texture indices.
#[test]
fn issue9_panic_reading_uffizi_textures() {
    let texture = include_ktx!("uffizi_rgba16f_cube.ktx");

    dbg!(texture);
    assert_eq!(texture.mipmap_levels(), 10);

    assert_eq!(texture.textures().count(), 10);

    for data in texture.textures() {
        dbg!(data.len());
    }
}

// #9 no panic in runtime read version, but worth keeping for regression testing.
#[test]
fn issue9_panic_reading_uffizi_textures_runtime() {
    let ktx_file = BufReader::new(File::open("tests/uffizi_rgba16f_cube.ktx").unwrap());
    let texture = ktx::Decoder::new(ktx_file).unwrap();

    dbg!(&texture);

    assert_eq!(texture.mipmap_levels(), 10);

    let levels: Vec<_> = texture.read_textures().collect();

    assert_eq!(levels.len(), 10);

    for data in levels {
        dbg!(data.len());
    }
}
