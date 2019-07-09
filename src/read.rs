use crate::header::*;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::{
    fmt,
    io::{self, Read},
};

/// KTX texture storage format reader. Useful when reading from a file and/or compressed data.
/// Provides [`KtxInfo`](../header/trait.KtxInfo.html).
///
/// # Example
/// ```
/// # use std::{io::BufReader, fs::File};
/// # fn main() -> std::io::Result<()> {
/// use ktx::*;
/// # let mut buf_reader = BufReader::new(File::open("tests/babg-bc3.ktx")?);
/// let mut decoder = ktx::Decoder::new(buf_reader)?;
///
/// assert_eq!(decoder.pixel_width(), 260);
/// let texture_levels: Vec<Vec<u8>> = decoder.read_textures().collect();
/// # Ok(()) }
/// ```
#[derive(Clone, Copy)]
pub struct KtxDecoder<R> {
    header: KtxHeader,
    data: R,
}

impl<R> AsRef<KtxHeader> for KtxDecoder<R> {
    #[inline]
    fn as_ref(&self) -> &KtxHeader {
        &self.header
    }
}

impl<R> fmt::Debug for KtxDecoder<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("KtxDecoder").field("header", &self.header).finish()
    }
}

impl<R: io::Read> KtxDecoder<R> {
    /// Reads KTX header data and returns a `KtxDecoder`.
    #[inline]
    pub fn new(mut data: R) -> io::Result<Self> {
        let mut header_data = [0; 64];
        data.read_exact(&mut header_data)?;
        let header = KtxHeader::new(&header_data);
        Ok(Self { header, data })
    }

    /// Consumes the `KtxDecoder` to returns an iterator reading texture levels starting at level 0.
    #[inline]
    pub fn read_textures(self) -> Textures<R> {
        Textures { header: self.header, data: self.data, next_level: 0 }
    }

    /// Returns `KtxHeader`. Useful if this info is desired after consuming the `KtxDecoder`.
    ///
    /// # Example
    /// ```
    /// # use std::{io::BufReader, fs::File};
    /// # use ktx::*;
    /// # let mut reader = BufReader::new(File::open("tests/babg-bc3.ktx").unwrap());
    /// # let mut decoder = ktx::Decoder::new(reader).unwrap();
    /// let ktx_header = decoder.header();
    /// assert_eq!(ktx_header.pixel_width(), 260);
    /// ```
    #[inline]
    pub fn header(&self) -> KtxHeader {
        self.header
    }
}

/// Iterator that reads texture level data into `Vec<u8>`.
#[derive(Debug)]
pub struct Textures<R> {
    header: KtxHeader,
    data: R,
    next_level: u32,
}

impl<R: io::Read> Iterator for Textures<R> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_level >= self.header.mipmap_levels() {
            None
        } else {
            // skip key-value data
            if self.next_level == 0 && self.header.bytes_of_key_value_data() != 0 {
                let mut discard = Vec::with_capacity(self.header.bytes_of_key_value_data() as _);
                self.data
                    .by_ref()
                    .take(self.header.bytes_of_key_value_data() as _)
                    .read_to_end(&mut discard)
                    .ok()?;
            }

            self.next_level += 1;
            let level_len = {
                let mut len = [0; 4];
                self.data.read_exact(&mut len).ok()?;
                if self.header.big_endian() {
                    BigEndian::read_u32(&len)
                } else {
                    LittleEndian::read_u32(&len)
                }
            };

            let mut level = Vec::with_capacity(level_len as _);
            self.data.by_ref().take(level_len as _).read_to_end(&mut level).ok()?;
            Some(level)
        }
    }
}

impl<R: io::Read> std::iter::FusedIterator for Textures<R> {}
