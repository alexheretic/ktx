use crate::header::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use core::{fmt, ops::Deref};

/// KTX texture storage format data stored in a complete slice.
/// Provides [`KtxInfo`](../header/trait.KtxInfo.html).
///
/// See the [specification](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
///
/// # Example
/// ```
/// # use ktx::*;
/// let image: Ktx<_> = include_ktx!("../tests/babg-bc3.ktx");
/// let texture_levels = image.textures().collect::<Vec<_>>();
/// ```
#[derive(Clone, Copy)]
pub struct Ktx<D> {
    header: KtxHeader,
    ktx_data: D,
    texture_start: u32,
}

impl<D> AsRef<KtxHeader> for Ktx<D> {
    #[inline]
    fn as_ref(&self) -> &KtxHeader {
        &self.header
    }
}

impl<D> fmt::Debug for Ktx<D> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Ktx")
            .field("header", &self.header)
            .finish()
    }
}

impl<D> Ktx<D>
where
    D: Deref<Target = [u8]>,
{
    /// Parses a complete KTX data slice and returns a `Ktx` instance.
    #[inline]
    pub fn new(ktx_data: D) -> Self {
        let header = KtxHeader::new(&ktx_data);
        let texture_start = 64 + header.bytes_of_key_value_data();
        Self {
            header,
            ktx_data,
            texture_start,
        }
    }

    /// Returns texture data at the input level, starting at `0`.
    ///
    /// # Panics
    ///
    /// Input level is >= the `mipmap_levels` value.
    #[inline]
    pub fn texture_level(&self, level: u32) -> Texture<'_> {
        self.textures().nth(level as _).expect("invalid level")
    }

    /// Returns an iterator over the texture levels starting at level 0.
    #[inline]
    pub fn textures(&self) -> Textures<'_, D> {
        Textures {
            parent: self,
            next_level: 0,
            level_end: self.texture_start as _,
        }
    }
}

impl<D> From<D> for Ktx<D>
where
    D: Deref<Target = [u8]>,
{
    #[inline]
    fn from(d: D) -> Self {
        Ktx::new(d)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Texture<'a> {
    TwoDim(&'a [u8]),
    Cubemap {
        all: &'a [u8],
        x: &'a [u8],
        x_neg: &'a [u8],
        y: &'a [u8],
        y_neg: &'a [u8],
        z: &'a [u8],
        z_neg: &'a [u8],
    },
}

impl<'a> std::ops::Deref for Texture<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::TwoDim(slice) => slice,
            Self::Cubemap { all, .. } => all,
        }
    }
}

/// Iterator over texture level data.
#[derive(Debug)]
pub struct Textures<'a, D> {
    parent: &'a Ktx<D>,
    next_level: u32,
    level_end: usize,
}

impl<'a, D> Iterator for Textures<'a, D>
where
    D: Deref<Target = [u8]>,
{
    type Item = Texture<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_level >= self.parent.mipmap_levels() {
            None
        } else {
            self.next_level += 1;

            let l_end = self.level_end;
            let next_tex_len = if self.parent.big_endian() {
                BigEndian::read_u32(&self.parent.ktx_data[l_end..l_end + 4])
            } else {
                LittleEndian::read_u32(&self.parent.ktx_data[l_end..l_end + 4])
            };

            if self.parent.array_elements() == 0 && self.parent.faces() == 6 {
                self.level_end = l_end + 4 + (6 * next_tex_len as usize);

                let level_begin = l_end + 4;
                let next_tex_len = next_tex_len as usize;
                Some(Texture::Cubemap {
                    all: &self.parent.ktx_data[level_begin..self.level_end],
                    x: &self.parent.ktx_data[level_begin..(level_begin + next_tex_len)],
                    x_neg: &self.parent.ktx_data
                        [level_begin + next_tex_len..(level_begin + (next_tex_len * 2))],
                    y: &self.parent.ktx_data
                        [level_begin + (next_tex_len * 2)..(level_begin + (next_tex_len * 3))],
                    y_neg: &self.parent.ktx_data
                        [level_begin + (next_tex_len * 3)..(level_begin + (next_tex_len * 4))],
                    z: &self.parent.ktx_data
                        [level_begin + (next_tex_len * 4)..(level_begin + (next_tex_len * 5))],
                    z_neg: &self.parent.ktx_data
                        [level_begin + (next_tex_len * 5)..(level_begin + (next_tex_len * 6))],
                })
            } else {
                self.level_end = l_end + 4 + next_tex_len as usize;
                Some(Texture::TwoDim(
                    &self.parent.ktx_data[l_end + 4..self.level_end],
                ))
            }
        }
    }
}

impl<D> core::iter::FusedIterator for Textures<'_, D> where D: Deref<Target = [u8]> {}

/// Wrapper for `include_bytes!` returning `Ktx<'static [u8]>`
///
/// # Example
/// ```
/// use ktx::{include_ktx, Ktx};
/// let image: Ktx<&'static [u8]> = include_ktx!("../tests/babg-bc3.ktx");
/// ```
#[macro_export]
macro_rules! include_ktx {
    ($path:tt) => {
        $crate::Ktx::new(include_bytes!($path) as &[u8])
    };
}
