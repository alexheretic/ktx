use ktx::*;
use std::{
    fs::File,
    io::{self, BufReader},
    sync::Arc,
};

#[test]
fn include_logo_example() {
    let ktx = include_ktx!("babg-bc3.ktx");

    assert!(!ktx.big_endian(), "!big_endian");
    assert_eq!(ktx.gl_type(), 0, "gl_type");
    assert_eq!(ktx.gl_type_size(), 1, "gl_type_size");
    assert_eq!(ktx.gl_format(), 0, "gl_format");
    assert_eq!(ktx.gl_internal_format(), 33779, "gl_internal_format");
    assert_eq!(
        ktx.gl_base_internal_format(),
        6408,
        "gl_base_internal_format"
    );
    assert_eq!(ktx.pixel_width(), 260, "pixel_width");
    assert_eq!(ktx.pixel_height(), 200, "pixel_height");
    assert_eq!(ktx.pixel_depth(), 0, "pixel_depth");
    assert_eq!(ktx.array_elements(), 0, "array_elements");
    assert_eq!(ktx.faces(), 1, "faces");
    assert_eq!(ktx.mipmap_levels(), 8, "mipmap_levels");
    assert_eq!(ktx.bytes_of_key_value_data(), 0, "bytes_of_key_value_data");
}

#[test]
fn read_logo_example() -> io::Result<()> {
    let ktx = ktx::Decoder::new(BufReader::new(File::open("tests/babg-bc3.ktx")?))?;

    assert!(!ktx.big_endian(), "!big_endian");
    assert_eq!(ktx.gl_type(), 0, "gl_type");
    assert_eq!(ktx.gl_type_size(), 1, "gl_type_size");
    assert_eq!(ktx.gl_format(), 0, "gl_format");
    assert_eq!(ktx.gl_internal_format(), 33779, "gl_internal_format");
    assert_eq!(
        ktx.gl_base_internal_format(),
        6408,
        "gl_base_internal_format"
    );
    assert_eq!(ktx.pixel_width(), 260, "pixel_width");
    assert_eq!(ktx.pixel_height(), 200, "pixel_height");
    assert_eq!(ktx.pixel_depth(), 0, "pixel_depth");
    assert_eq!(ktx.array_elements(), 0, "array_elements");
    assert_eq!(ktx.faces(), 1, "faces");
    assert_eq!(ktx.mipmap_levels(), 8, "mipmap_levels");
    assert_eq!(ktx.bytes_of_key_value_data(), 0, "bytes_of_key_value_data");
    Ok(())
}

#[test]
fn owned_logo_example() {
    let owned_ktx_data: Arc<[u8]> = Arc::from(include_bytes!("babg-bc3.ktx").to_vec());
    let ktx: Ktx<Arc<[u8]>> = Ktx::from(owned_ktx_data);

    assert_eq!(ktx.pixel_width(), 260, "pixel_width");
    assert_eq!(ktx.pixel_height(), 200, "pixel_height");
}

const LOGO_LEVEL_0_BLAKE: &str = "981b99f24330d900598dbee39ba512e523f81d53b49c5d1c0dbfc5f6b4bd98d2";
const LOGO_LEVEL_1_BLAKE: &str = "20d3c9d28dad531659c4047e1cce3d9911a74e95c5f16022fdeea2795a1e61e7";
const LOGO_LEVEL_2_BLAKE: &str = "abfdae2adc718bec8ff20b9337c0fe08b619087e07156fff6d8e4409062c679d";
const LOGO_LEVEL_3_BLAKE: &str = "0b792177e99af288a5f10fb34f63837edb46ae6f7441760a3e1a28457c3a6f4a";
const LOGO_LEVEL_4_BLAKE: &str = "b97b9e45f228e593b3b324ea27ed9581f58c749e078d1cbdc5676212804545c7";
const LOGO_LEVEL_5_BLAKE: &str = "ad3dc3f0254cd9804b0d60c4f43e46f841f24d0ecc4e2d9d7967c0002209848f";
const LOGO_LEVEL_6_BLAKE: &str = "f5ea633e4099163a9b6ac2d22491ab38bdf8062a5c70424bfac9233dc06e3d0a";
const LOGO_LEVEL_7_BLAKE: &str = "06d5aa4be4a9d14007e52dcdfa995e47cdb44a3541e6e97d8021e690aaa75824";

#[test]
fn include_logo_example_textures() {
    let ktx = include_ktx!("babg-bc3.ktx");
    let mut textures = ktx.textures();

    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_1_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_2_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_3_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_4_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_5_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_6_BLAKE
    );
    assert_eq!(
        &blake3::hash(textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_7_BLAKE
    );
    assert_eq!(textures.next(), None);
}

#[test]
fn read_logo_example_textures() -> io::Result<()> {
    let ktx = ktx::Decoder::new(BufReader::new(File::open("tests/babg-bc3.ktx")?))?;
    let mut textures = ktx.read_textures();

    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_1_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_2_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_3_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_4_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_5_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_6_BLAKE
    );
    assert_eq!(
        &blake3::hash(&textures.next().unwrap()).to_hex(),
        LOGO_LEVEL_7_BLAKE
    );
    assert_eq!(textures.next(), None);
    Ok(())
}

#[test]
fn logo_example_texture_level() {
    let ktx = include_ktx!("babg-bc3.ktx");

    assert_eq!(
        &blake3::hash(ktx.texture_level(0)).to_hex(),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        &blake3::hash(ktx.texture_level(4)).to_hex(),
        LOGO_LEVEL_4_BLAKE
    );
}

#[test]
fn logo_example_debug() {
    let dbg_string = format!("{:?}", include_ktx!("babg-bc3.ktx"));
    assert_eq!(
        &dbg_string,
        "Ktx { header: KtxHeader { big_endian: false, gl_type: 0, gl_type_size: 1, gl_format: 0, gl_internal_format: 33779, gl_base_internal_format: 6408, pixel_width: 260, pixel_height: 200, pixel_depth: 0, array_elements: 0, faces: 1, mipmap_levels: 8, bytes_of_key_value_data: 0 } }"
    );
}

#[test]
fn uffizi_6face_include() {
    let ktx = include_ktx!("uffizi_rgba16f_cube.ktx");

    assert!(!ktx.big_endian(), "!big_endian");
    assert_eq!(ktx.gl_type(), 5131, "gl_type");
    assert_eq!(ktx.gl_type_size(), 2, "gl_type_size");
    assert_eq!(ktx.gl_format(), 6408, "gl_format");
    assert_eq!(ktx.gl_internal_format(), 34842, "gl_internal_format");
    assert_eq!(
        ktx.gl_base_internal_format(),
        6408,
        "gl_base_internal_format"
    );
    assert_eq!(ktx.pixel_width(), 512, "pixel_width");
    assert_eq!(ktx.pixel_height(), 512, "pixel_height");
    assert_eq!(ktx.pixel_depth(), 0, "pixel_depth");
    assert_eq!(ktx.array_elements(), 0, "array_elements");
    assert_eq!(ktx.faces(), 6, "faces");
    assert_eq!(ktx.mipmap_levels(), 10, "mipmap_levels");
    assert_eq!(ktx.bytes_of_key_value_data(), 0, "bytes_of_key_value_data");

    assert_eq!(ktx.textures().count(), 10, "ktx.textures().count()");
}

#[test]
fn uffizi_6face_read() {
    let ktx_file = BufReader::new(File::open("tests/uffizi_rgba16f_cube.ktx").unwrap());
    let ktx = ktx::Decoder::new(ktx_file).unwrap();

    assert!(!ktx.big_endian(), "!big_endian");
    assert_eq!(ktx.gl_type(), 5131, "gl_type");
    assert_eq!(ktx.gl_type_size(), 2, "gl_type_size");
    assert_eq!(ktx.gl_format(), 6408, "gl_format");
    assert_eq!(ktx.gl_internal_format(), 34842, "gl_internal_format");
    assert_eq!(
        ktx.gl_base_internal_format(),
        6408,
        "gl_base_internal_format"
    );
    assert_eq!(ktx.pixel_width(), 512, "pixel_width");
    assert_eq!(ktx.pixel_height(), 512, "pixel_height");
    assert_eq!(ktx.pixel_depth(), 0, "pixel_depth");
    assert_eq!(ktx.array_elements(), 0, "array_elements");
    assert_eq!(ktx.faces(), 6, "faces");
    assert_eq!(ktx.mipmap_levels(), 10, "mipmap_levels");
    assert_eq!(ktx.bytes_of_key_value_data(), 0, "bytes_of_key_value_data");

    assert_eq!(
        ktx.read_textures().count(),
        10,
        "ktx.read_textures().count()"
    );
}
