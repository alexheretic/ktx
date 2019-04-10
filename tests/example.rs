use blake2::{Blake2s, Digest};
use ktx::{include_ktx, Ktx};

#[test]
fn logo_example() {
    let ktx = include_ktx!("logo-bc3.ktx");

    assert_eq!(ktx.gl_type, 0, "gl_type");
    assert_eq!(ktx.gl_type_size, 1, "gl_type_size");
    assert_eq!(ktx.gl_format, 0, "gl_format");
    assert_eq!(ktx.gl_internal_format, 33779, "gl_internal_format");
    assert_eq!(ktx.gl_base_internal_format, 6408, "gl_base_internal_format");
    assert_eq!(ktx.pixel_width, 260, "pixel_width");
    assert_eq!(ktx.pixel_height, 200, "pixel_height");
    assert_eq!(ktx.pixel_depth, 0, "pixel_depth");
    assert_eq!(ktx.array_elements, 0, "array_elements");
    assert_eq!(ktx.faces, 1, "faces");
    assert_eq!(ktx.mipmap_levels, 8, "mipmap_levels");
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
fn logo_example_textures() {
    let ktx = include_ktx!("logo-bc3.ktx");
    let mut textures = ktx.textures();

    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_0_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_1_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_2_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_3_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_4_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_5_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_6_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&textures.next().unwrap())), LOGO_LEVEL_7_BLAKE);
    assert_eq!(textures.next(), None);
}

#[test]
fn logo_example_texture_level() {
    let ktx = include_ktx!("logo-bc3.ktx");

    assert_eq!(format!("{:x}", Blake2s::digest(&ktx.texture_level(0))), LOGO_LEVEL_0_BLAKE);
    assert_eq!(format!("{:x}", Blake2s::digest(&ktx.texture_level(4))), LOGO_LEVEL_4_BLAKE);
}
