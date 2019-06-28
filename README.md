ktx
[![crates.io](https://img.shields.io/crates/v/ktx.svg)](https://crates.io/crates/ktx)
[![Documentation](https://docs.rs/ktx/badge.svg)](https://docs.rs/ktx)
==========

KTX texture storage format parsing.

Parses byte data according to [https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec](https://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec).

```rust
use ktx::*;

// Include & use static ktx data
let image: Ktx<'static> = include_ktx!("../tests/babg-bc3.ktx");
assert_eq!(image.pixel_width(), 260);

// Read ktx data
let decoder = ktx::Decoder::new(buf_reader)?;
assert_eq!(decoder.pixel_width(), 260);
```

## Minimum supported rust compiler
This crate is maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
