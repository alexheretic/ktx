ktx
[![crates.io](https://img.shields.io/crates/v/ktx.svg)](https://crates.io/crates/ktx)
[![Documentation](https://docs.rs/ktx/badge.svg)](https://docs.rs/ktx)
==========

KTX v1 texture storage format parsing.

Parses byte data according to [https://www.khronos.org/registry/KTX/specs/1.0/ktxspec_v1.html](https://www.khronos.org/registry/KTX/specs/1.0/ktxspec_v1.html).

```rust
// Include & use static ktx data
use ktx::{Ktx, include_ktx, KtxInfo};

let image: Ktx<_> = include_ktx!("../tests/babg-bc3.ktx");
assert_eq!(image.pixel_width(), 260);
```

```rust
// Read ktx data at runtime
use ktx::KtxInfo;

let decoder = ktx::Decoder::new(buf_reader)?;
assert_eq!(decoder.pixel_width(), 260);
```

## Minimum supported rust compiler
This crate is maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
