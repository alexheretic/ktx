use blake2::{Blake2s, Digest};
use ktx::*;
use std::{fs, io, sync::Arc};

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
    let ktx = ktx::Decoder::new(io::BufReader::new(fs::File::open("tests/babg-bc3.ktx")?))?;

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

const LOGO_LEVEL_0_BLAKE: &str = "17ae9dcdc7b7f8c38a66fe00ab92759fde35f74cde2aa52449c2ecbca835a51b";
const LOGO_LEVEL_1_BLAKE: &str = "ae05372daa8bb0d45de4431db106e107266307e5a37b51825f10618a9605ee1b";
const LOGO_LEVEL_2_BLAKE: &str = "52ed1d989b8dca91538d68e1077d303a95baf6068d240316bc18c3d8ef5625ea";
const LOGO_LEVEL_3_BLAKE: &str = "3869a1ccc011b1c74255f2a7ccc2a2c5118a492599aea758c59e4b4b80e3bd6a";
const LOGO_LEVEL_4_BLAKE: &str = "5cca1fd6eeb47922490392884e2a5a177f791a37da64109705127b092f4073bc";
const LOGO_LEVEL_5_BLAKE: &str = "2e280b7441e1576d93a7cdf97f67e3fcecdefe209a34c10e9adf5a4158351884";
const LOGO_LEVEL_6_BLAKE: &str = "22703f682061beb020f2316cbcad901268f1ad7869fd1892062b9d50b92508c0";
const LOGO_LEVEL_7_BLAKE: &str = "971ccf807344ecef5e43d10bcd0bd9260d35b1632548d045ba2cbbbf8a50075a";

#[test]
fn include_logo_example_textures() {
    let ktx = include_ktx!("babg-bc3.ktx");
    let mut textures = ktx.textures();

    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_1_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_2_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_3_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_4_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_5_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_6_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_7_BLAKE
    );
    assert_eq!(textures.next(), None);
}

#[test]
fn read_logo_example_textures() -> io::Result<()> {
    let ktx = ktx::Decoder::new(io::BufReader::new(fs::File::open("tests/babg-bc3.ktx")?))?;
    let mut textures = ktx.read_textures();

    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_1_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_2_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_3_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_4_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_5_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_6_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&textures.next().unwrap())),
        LOGO_LEVEL_7_BLAKE
    );
    assert_eq!(textures.next(), None);
    Ok(())
}

#[test]
fn logo_example_texture_level() {
    let ktx = include_ktx!("babg-bc3.ktx");

    assert_eq!(
        format!("{:x}", Blake2s::digest(&ktx.texture_level(0))),
        LOGO_LEVEL_0_BLAKE
    );
    assert_eq!(
        format!("{:x}", Blake2s::digest(&ktx.texture_level(4))),
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
