//! KTX v1 texture storage format parsing.
//!
//! Parses byte data according to
//! [https://www.khronos.org/registry/KTX/specs/1.0/ktxspec_v1.html](https://www.khronos.org/registry/KTX/specs/1.0/ktxspec_v1.html).
//!
//! # Example: Include at compile time
//! ```
//! # fn main() -> std::io::Result<()> {
//! use ktx::{include_ktx, Ktx, KtxInfo};
//!
//! // Include & use static ktx data
//! let image: Ktx<_> = include_ktx!("../tests/babg-bc3.ktx");
//! assert_eq!(image.pixel_width(), 260);
//! # Ok(()) }
//! ```
//!
//! # Example: Read at runtime
//! ```
//! # fn main() -> std::io::Result<()> {
//! # use std::{io::BufReader, fs::File};
//! use ktx::KtxInfo;
//!
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
