use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::fmt;

const KTX_IDENTIFIER: [u8; 12] =
    [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x31, 0x31, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];

/// KTX texture storage format
///
/// https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec
#[derive(Clone, Copy)]
pub struct Ktx<'a> {
    big_endian: bool,
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
    bytes_of_key_value_data: u32,
    // key_value_data: &'a [u8],
    texture_data: &'a [u8],
}

impl<'data> Ktx<'data> {
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

    #[inline]
    pub fn texture_level(&self, level: u32) -> &'data [u8] {
        self.textures().nth(level as _).expect("invalid level")
    }

    #[inline]
    pub fn textures(&self) -> Textures<'data, '_> {
        Textures { parent: self, next_level: 0, level_end: 0 }
    }
}

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

#[macro_export]
macro_rules! include_ktx {
    ($path:tt) => {
        Ktx::new(include_bytes!($path) as _)
    };
}
