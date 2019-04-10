//! KTX texture storage format parsing.
//!
//! Parses byte data according to
//! [https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
//!
//! # Example
//!
//! ```
//! use ktx::{include_ktx, Ktx};
//!
//! let image: Ktx<'static> = include_ktx!("../tests/babg-bc3.ktx");
//! assert_eq!(image.pixel_width, 260);
//! ```

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::fmt;

const KTX_IDENTIFIER: [u8; 12] =
    [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];

/// KTX texture storage format data.
///
/// See the [specification](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
#[derive(Clone, Copy)]
pub struct Ktx<'a> {
    big_endian: bool,
    /// For compressed textures, glType must equal 0. For uncompressed textures, glType specifies the type parameter passed to glTex{,Sub}Image*D, usually one of the values from table 8.2 of the OpenGL 4.4 specification [OPENGL44](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/#refsGL44) (UNSIGNED_BYTE, UNSIGNED_SHORT_5_6_5, etc.)
    pub gl_type: u32,
    /// glTypeSize specifies the data type size that should be used when endianness conversion is required for the texture data stored in the file. If glType is not 0, this should be the size in bytes corresponding to glType. For texture data which does not depend on platform endianness, including compressed texture data, glTypeSize must equal 1.
    pub gl_type_size: u32,
    /// For compressed textures, glFormat must equal 0. For uncompressed textures, glFormat specifies the format parameter passed to glTex{,Sub}Image*D, usually one of the values from table 8.3 of the OpenGL 4.4 specification [OPENGL44](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/#refsGL44) (RGB, RGBA, BGRA, etc.)
    pub gl_format: u32,
    /// For compressed textures, glInternalFormat must equal the compressed internal format, usually one of the values from table 8.14 of the OpenGL 4.4 specification [OPENGL44]. For uncompressed textures, glInternalFormat specifies the internalformat parameter passed to glTexStorage*D or glTexImage*D, usually one of the sized internal formats from tables 8.12 & 8.13 of the OpenGL 4.4 specification [OPENGL44]. The sized format should be chosen to match the bit depth of the data provided. glInternalFormat is used when loading both compressed and uncompressed textures, except when loading into a context that does not support sized formats, such as an unextended OpenGL ES 2.0 context where the internalformat parameter is required to have the same value as the format parameter.
    pub gl_internal_format: u32,
    /// For both compressed and uncompressed textures, glBaseInternalFormat specifies the base internal format of the texture, usually one of the values from table 8.11 of the OpenGL 4.4 specification [OPENGL44] (RGB, RGBA, ALPHA, etc.). For uncompressed textures, this value will be the same as glFormat and is used as the internalformat parameter when loading into a context that does not support sized formats, such as an unextended OpenGL ES 2.0 context.
    pub gl_base_internal_format: u32,
    /// The width of the texture image for level 0, in pixels. No rounding to block sizes should be applied for block compressed textures.
    pub pixel_width: u32,
    /// The height of the texture image for level 0, in pixels. No rounding to block sizes should be applied for block compressed textures.
    ///
    /// For 1D textures this must be 0.
    pub pixel_height: u32,
    /// The depth of the texture image for level 0, in pixels. No rounding to block sizes should be applied for block compressed textures.
    ///
    /// For 1D textures this must be 0. For 2D and cube textures this must be 0.
    pub pixel_depth: u32,
    /// numberOfArrayElements specifies the number of array elements. If the texture is not an array texture, numberOfArrayElements must equal 0.
    pub array_elements: u32,
    /// numberOfFaces specifies the number of cubemap faces. For cubemaps and cubemap arrays this should be 6. For non cubemaps this should be 1. Cube map faces are stored in the order: +X, -X, +Y, -Y, +Z, -Z.
    ///
    /// Due to GL_OES_compressed_paletted_texture [OESCPT] not defining the interaction between cubemaps and its GL_PALETTE* formats, if `glInternalFormat` is one of its GL_PALETTE* format, numberOfFaces must be 1
    pub faces: u32,
    /// numberOfMipmapLevels must equal 1 for non-mipmapped textures. For mipmapped textures, it equals the number of mipmaps. Mipmaps are stored in order from largest size to smallest size. The first mipmap level is always level 0. A KTX file does not need to contain a complete mipmap pyramid. If numberOfMipmapLevels equals 0, it indicates that a full mipmap pyramid should be generated from level 0 at load time (this is usually not allowed for compressed formats).
    pub mipmap_levels: u32,
    bytes_of_key_value_data: u32,
    // key_value_data: &'a [u8],
    texture_data: &'a [u8],
}

impl<'data> Ktx<'data> {
    /// Parses KTX header data and returns a `Ktx` instance.
    pub fn new(data: &'data [u8]) -> Self {
        debug_assert_eq!(&data[..12], &KTX_IDENTIFIER, "Not KTX");

        let big_endian = data[12] == 4;

        let mut vals: [u32; 12] = <_>::default();
        if big_endian {
            BigEndian::read_u32_into(&data[16..64], &mut vals);
        } else {
            LittleEndian::read_u32_into(&data[16..64], &mut vals);
        }

        let kv_len = vals[11] as usize;
        // let key_value_data = &data[64..64 + kv_len];
        let texture_data = &data[64 + kv_len..];
        Self {
            big_endian,
            gl_type: vals[0],
            gl_type_size: vals[1],
            gl_format: vals[2],
            gl_internal_format: vals[3],
            gl_base_internal_format: vals[4],
            pixel_width: vals[5],
            pixel_height: vals[6],
            pixel_depth: vals[7],
            array_elements: vals[8],
            faces: vals[9],
            mipmap_levels: vals[10],
            bytes_of_key_value_data: vals[11],
            // key_value_data,
            texture_data,
        }
    }

    /// Returns texture data at the input level, starting at `0`.
    ///
    /// # Panics
    ///
    /// Input level is >= the `mipmap_levels` value.
    #[inline]
    pub fn texture_level(&self, level: u32) -> &'data [u8] {
        self.textures().nth(level as _).expect("invalid level")
    }

    /// Returns an iterator over the texture levels starting at level 0.
    #[inline]
    pub fn textures(&self) -> Textures<'data, '_> {
        Textures { parent: self, next_level: 0, level_end: 0 }
    }
}

/// Iterator over texture level data.
#[derive(Debug)]
pub struct Textures<'data, 'a> {
    parent: &'a Ktx<'data>,
    next_level: u32,
    level_end: usize,
}

impl<'data> Iterator for Textures<'data, '_> {
    type Item = &'data [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_level >= self.parent.mipmap_levels {
            None
        } else {
            self.next_level += 1;

            let l_end = self.level_end;
            let next_lvl_len = if self.parent.big_endian {
                BigEndian::read_u32(&self.parent.texture_data[l_end..l_end + 4])
            } else {
                LittleEndian::read_u32(&self.parent.texture_data[l_end..l_end + 4])
            };
            self.level_end = l_end + 4 + next_lvl_len as usize;
            Some(&self.parent.texture_data[l_end + 4..self.level_end])
        }
    }
}

impl std::iter::FusedIterator for Textures<'_, '_> {}

impl fmt::Debug for Ktx<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Ktx")
            .field("big_endian", &self.big_endian)
            .field("gl_type", &self.gl_type)
            .field("gl_type_size", &self.gl_type_size)
            .field("gl_format", &self.gl_format)
            .field("gl_internal_format", &self.gl_internal_format)
            .field("gl_base_internal_format", &self.gl_base_internal_format)
            .field("pixel_width", &self.pixel_width)
            .field("pixel_height", &self.pixel_height)
            .field("pixel_depth", &self.pixel_depth)
            .field("array_elements", &self.array_elements)
            .field("faces", &self.faces)
            .field("mipmap_levels", &self.mipmap_levels)
            .field("bytes_of_key_value_data", &self.bytes_of_key_value_data)
            .finish()
    }
}

/// Wrapper for `include_bytes!` returning `Ktx<'static>`
#[macro_export]
macro_rules! include_ktx {
    ($path:tt) => {
        Ktx::new(include_bytes!($path) as _)
    };
}
