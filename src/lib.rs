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
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::cast_lossless)]

pub mod header;
#[cfg(feature = "std")]
pub mod read;
pub mod slice;

pub use header::KtxInfo;
#[cfg(feature = "std")]
pub use read::KtxDecoder as Decoder;
pub use slice::Ktx;
