//! KTX texture storage format parsing.
//!
//! Parses byte data according to
//! [https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
//!
//! # Example
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use ktx::*;
//!
//! // Include & use static ktx data
//! let image: Ktx<'static> = include_ktx!("../tests/babg-bc3.ktx");
//! assert_eq!(image.pixel_width(), 260);
//!
//! // Read ktx data
//! # use std::{io::BufReader, fs::File};
//! # use ktx::*;
//! # let mut buf_reader = BufReader::new(File::open("tests/babg-bc3.ktx").unwrap());
//! let decoder = ktx::Decoder::new(buf_reader)?;
//! assert_eq!(decoder.pixel_width(), 260);
//! # Ok(()) }
//! ```
#![allow(clippy::cast_lossless)]

#[macro_use]
pub mod header;
pub mod read;
pub mod slice;

pub use header::KtxInfo;
pub use read::KtxDecoder as Decoder;
pub use slice::Ktx;
