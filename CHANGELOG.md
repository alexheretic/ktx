# 0.3.2
* Support cubemap textures. For such textures each level will contain all 6 faces in order: +X, -X, +Y, -Y, +Z, -Z.

# 0.3.1
* Add new default feature `std` which can be disabled to allow no-std usage.

# 0.3
* `Ktx` now uses a generic `D: Deref<Target = [u8]>` instead of requiring a slice. This supports owned data usage in addition to slice usage. The texture slice lifetime is now the shorter `Ktx` lifetime because of this.
* Remove internal macros instead `AsRef<KtxHeader>` now provides `KtxInfo` allowing `Ktx`, `KtxHeader`, `KtxDecoder` to all implement `KtxInfo`.

# 0.2
* Add `ktx::Decoder` useful when reading from a file and/or compressed data.
* Separate header logic into `KtxInfo` trait now provided by methods instead of direct field access.

# 0.1
Initial release supporting KTX formatted byte data.