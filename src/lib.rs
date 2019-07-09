//! KTX texture storage format parsing.
//!
//! Parses byte data according to
//! [https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).
//!
//! # Example
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use ktx::{Ktx, include_ktx, KtxInfo};
//!
//! // Include & use static ktx data
//! let image: Ktx<_> = include_ktx!("../tests/babg-bc3.ktx");
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
pub mod slice;
pub mod read;

pub use slice::Ktx;
pub use header::KtxInfo;
pub use read::KtxDecoder as Decoder;
