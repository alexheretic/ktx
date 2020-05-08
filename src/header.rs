use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub(crate) const KTX_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

/// KTX texture storage format parameters.
///
/// See the [specification](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
pub trait KtxInfo {
    /// endianness contains the number 0x04030201 written as a 32 bit integer. If the file is little
    /// endian then this is represented as the bytes 0x01 0x02 0x03 0x04. If the file is big endian
    /// then this is represented as the bytes 0x04 0x03 0x02 0x01. When reading endianness as a 32
    /// bit integer produces the value 0x04030201 then the endianness of the file matches the the
    /// endianness of the program that is reading the file and no conversion is necessary. When
    /// reading endianness as a 32 bit integer produces the value 0x01020304 then the endianness of
    /// the file is opposite the endianness of the program that is reading the file, and in that
    /// case the program reading the file must endian convert all header bytes and, if glTypeSize >
    /// 1, all texture data to the endianness of the program (i.e. a little endian program must
    /// convert from big endian, and a big endian program must convert to little endian).
    fn big_endian(&self) -> bool;
    /// For compressed textures, glType must equal 0. For uncompressed textures, glType specifies the type parameter passed to glTex{,Sub}Image*D, usually one of the values from table 8.2 of the OpenGL 4.4 specification [OPENGL44](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/#refsGL44) (UNSIGNED_BYTE, UNSIGNED_SHORT_5_6_5, etc.)
    fn gl_type(&self) -> u32;
    /// glTypeSize specifies the data type size that should be used when endianness conversion is
    /// required for the texture data stored in the file. If glType is not 0, this should be the
    /// size in bytes corresponding to glType. For texture data which does not depend on platform
    /// endianness, including compressed texture data, glTypeSize must equal 1.
    fn gl_type_size(&self) -> u32;
    /// For compressed textures, glFormat must equal 0. For uncompressed textures, glFormat specifies the format parameter passed to glTex{,Sub}Image*D, usually one of the values from table 8.3 of the OpenGL 4.4 specification [OPENGL44](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/#refsGL44) (RGB, RGBA, BGRA, etc.)
    fn gl_format(&self) -> u32;
    /// For compressed textures, glInternalFormat must equal the compressed internal format, usually
    /// one of the values from table 8.14 of the OpenGL 4.4 specification [OPENGL44]. For
    /// uncompressed textures, glInternalFormat specifies the internalformat parameter passed to
    /// glTexStorage*D or glTexImage*D, usually one of the sized internal formats from tables 8.12 &
    /// 8.13 of the OpenGL 4.4 specification [OPENGL44]. The sized format should be chosen to match
    /// the bit depth of the data provided. glInternalFormat is used when loading both compressed
    /// and uncompressed textures, except when loading into a context that does not support sized
    /// formats, such as an unextended OpenGL ES 2.0 context where the internalformat parameter is
    /// required to have the same value as the format parameter.
    fn gl_internal_format(&self) -> u32;
    /// For both compressed and uncompressed textures, glBaseInternalFormat specifies the base
    /// internal format of the texture, usually one of the values from table 8.11 of the OpenGL 4.4
    /// specification [OPENGL44] (RGB, RGBA, ALPHA, etc.). For uncompressed textures, this value
    /// will be the same as glFormat and is used as the internalformat parameter when loading into a
    /// context that does not support sized formats, such as an unextended OpenGL ES 2.0 context.
    fn gl_base_internal_format(&self) -> u32;
    /// The width of the texture image for level 0, in pixels. No rounding to block sizes should be
    /// applied for block compressed textures.
    fn pixel_width(&self) -> u32;
    /// The height of the texture image for level 0, in pixels. No rounding to block sizes should be
    /// applied for block compressed textures.
    ///
    /// For 1D textures this must be 0.
    fn pixel_height(&self) -> u32;
    /// The depth of the texture image for level 0, in pixels. No rounding to block sizes should be
    /// applied for block compressed textures.
    ///
    /// For 1D textures this must be 0. For 2D and cube textures this must be 0.
    fn pixel_depth(&self) -> u32;
    /// numberOfArrayElements specifies the number of array elements. If the texture is not an array
    /// texture, numberOfArrayElements must equal 0.
    fn array_elements(&self) -> u32;
    /// numberOfFaces specifies the number of cubemap faces. For cubemaps and cubemap arrays this
    /// should be 6. For non cubemaps this should be 1. Cube map faces are stored in the order: +X,
    /// -X, +Y, -Y, +Z, -Z.
    ///
    /// Due to GL_OES_compressed_paletted_texture [OESCPT] not defining the interaction between
    /// cubemaps and its GL_PALETTE* formats, if `glInternalFormat` is one of its GL_PALETTE*
    /// format, numberOfFaces must be 1
    fn faces(&self) -> u32;
    /// numberOfMipmapLevels must equal 1 for non-mipmapped textures. For mipmapped textures, it
    /// equals the number of mipmaps. Mipmaps are stored in order from largest size to smallest
    /// size. The first mipmap level is always level 0. A KTX file does not need to contain a
    /// complete mipmap pyramid. If numberOfMipmapLevels equals 0, it indicates that a full mipmap
    /// pyramid should be generated from level 0 at load time (this is usually not allowed for
    /// compressed formats).
    fn mipmap_levels(&self) -> u32;
    /// keyAndValueByteSize is the number of bytes of combined key and value data in one key/value
    /// pair following the header. This includes the size of the key, the NUL byte terminating the
    /// key, and all the bytes of data in the value. If the value is a UTF-8 string it should be NUL
    /// terminated and the keyAndValueByteSize should include the NUL character (but code that reads
    /// KTX files must not assume that value fields are NUL terminated). keyAndValueByteSize does
    /// not include the bytes in valuePadding.
    fn bytes_of_key_value_data(&self) -> u32;
}

/// KTX texture storage format header. Provides [`KtxInfo`](../header/trait.KtxInfo.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KtxHeader {
    pub big_endian: bool,
    pub gl_type: u32,
    pub gl_type_size: u32,
    pub gl_format: u32,
    pub gl_internal_format: u32,
    pub gl_base_internal_format: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub pixel_depth: u32,
    pub array_elements: u32,
    pub faces: u32,
    pub mipmap_levels: u32,
    pub bytes_of_key_value_data: u32,
}

impl KtxHeader {
    /// Reads first 64 bytes to parse KTX header data, returns a `KtxHeader`.
    pub fn new(first_64_bytes: &[u8]) -> Self {
        debug_assert!(first_64_bytes.len() >= 64);
        debug_assert_eq!(&first_64_bytes[..12], &KTX_IDENTIFIER, "Not KTX");

        let big_endian = first_64_bytes[12] == 4;

        let mut vals: [u32; 12] = <_>::default();
        if big_endian {
            BigEndian::read_u32_into(&first_64_bytes[16..64], &mut vals);
        } else {
            LittleEndian::read_u32_into(&first_64_bytes[16..64], &mut vals);
        }

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
        }
    }

    pub fn write(&self, first_64_bytes: &mut [u8]) {
        debug_assert!(first_64_bytes.len() >= 64);

        let mut vals: [u32; 13] = <_>::default();
        vals[0] = 0x04030201;
        vals[1] = self.gl_type;
        vals[2] = self.gl_type_size;
        vals[3] = self.gl_format;
        vals[4] = self.gl_internal_format;
        vals[5] = self.gl_base_internal_format;
        vals[6] = self.pixel_width;
        vals[7] = self.pixel_height;
        vals[8] = self.pixel_depth;
        vals[9] = self.array_elements;
        vals[10] = self.faces;
        vals[11] = self.mipmap_levels;
        vals[12] = self.bytes_of_key_value_data;

        (&mut first_64_bytes[0..KTX_IDENTIFIER.len()]).copy_from_slice(&KTX_IDENTIFIER);

        if self.big_endian {
            BigEndian::write_u32_into(
                &vals,
                &mut first_64_bytes[KTX_IDENTIFIER.len()
                    ..KTX_IDENTIFIER.len() + (vals.len() * std::mem::size_of::<u32>())],
            )
        } else {
            LittleEndian::write_u32_into(
                &vals,
                &mut first_64_bytes[KTX_IDENTIFIER.len()
                    ..KTX_IDENTIFIER.len() + (vals.len() * std::mem::size_of::<u32>())],
            );
        };
    }
}

impl AsRef<KtxHeader> for KtxHeader {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> KtxInfo for T
where
    T: AsRef<KtxHeader>,
{
    #[inline]
    fn big_endian(&self) -> bool {
        self.as_ref().big_endian
    }
    #[inline]
    fn gl_type(&self) -> u32 {
        self.as_ref().gl_type
    }
    #[inline]
    fn gl_type_size(&self) -> u32 {
        self.as_ref().gl_type_size
    }
    #[inline]
    fn gl_format(&self) -> u32 {
        self.as_ref().gl_format
    }
    #[inline]
    fn gl_internal_format(&self) -> u32 {
        self.as_ref().gl_internal_format
    }
    #[inline]
    fn gl_base_internal_format(&self) -> u32 {
        self.as_ref().gl_base_internal_format
    }
    #[inline]
    fn pixel_width(&self) -> u32 {
        self.as_ref().pixel_width
    }
    #[inline]
    fn pixel_height(&self) -> u32 {
        self.as_ref().pixel_height
    }
    #[inline]
    fn pixel_depth(&self) -> u32 {
        self.as_ref().pixel_depth
    }
    #[inline]
    fn array_elements(&self) -> u32 {
        self.as_ref().array_elements
    }
    #[inline]
    fn faces(&self) -> u32 {
        self.as_ref().faces
    }
    #[inline]
    fn mipmap_levels(&self) -> u32 {
        self.as_ref().mipmap_levels
    }
    #[inline]
    fn bytes_of_key_value_data(&self) -> u32 {
        self.as_ref().bytes_of_key_value_data
    }
}
