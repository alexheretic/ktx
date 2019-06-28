use crate::header::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::fmt;

/// KTX texture storage format data stored in a complete slice.
///
/// See the [specification](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
///
/// # Example
/// ```
/// use ktx::*;
/// let image: Ktx<'static> = include_ktx!("../tests/babg-bc3.ktx");
/// let texture_levels: Vec<&[u8]> = image.textures().collect();
/// ```
#[derive(Clone, Copy)]
pub struct Ktx<'data> {
    header: KtxHeader,
    texture_data: &'data [u8],
}

impl KtxInfo for Ktx<'_> {
    delegate_ktx_info!(header);
}

impl fmt::Debug for Ktx<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Ktx").field("header", &self.header).finish()
    }
}

impl<'data> Ktx<'data> {
    /// Parses a complete KTX data slice and returns a `Ktx` instance.
    #[inline]
    pub fn new(data: &'data [u8]) -> Self {
        let header = KtxHeader::new(data);
        let texture_data = &data[64 + header.bytes_of_key_value_data() as usize..];
        Self { header, texture_data }
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
        if self.next_level >= self.parent.mipmap_levels() {
            None
        } else {
            self.next_level += 1;

            let l_end = self.level_end;
            let next_lvl_len = if self.parent.big_endian() {
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

/// Wrapper for `include_bytes!` returning `Ktx<'static>`
///
/// # Example
/// ```
/// use ktx::include_ktx;
/// let image = include_ktx!("../tests/babg-bc3.ktx");
/// ```
#[macro_export]
macro_rules! include_ktx {
    ($path:tt) => {
        $crate::Ktx::new(include_bytes!($path) as _)
    };
}
