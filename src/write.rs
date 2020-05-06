use crate::header::{KtxHeader, KtxInfo, KTX_IDENTIFIER};
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub struct KtxBuilder {
    header: KtxHeader,
    levels: Vec<Vec<u8>>,
}
impl KtxBuilder {
    pub fn new(header: KtxHeader) -> Self {
        Self {
            header,
            levels: Vec::default(),
        }
    }

    pub fn with_level(mut self, texture: Vec<u8>) -> Self {
        self.add_level(texture);

        self
    }

    pub fn add_level(&mut self, texture: Vec<u8>) {
        self.levels.push(texture);
    }

    pub fn to_vec(self) -> Result<Vec<u8>, &'static str> {
        let KtxBuilder { header, levels } = self;

        // allocate the full size
        let size = KTX_IDENTIFIER.len()
            + std::mem::size_of::<KtxHeader>()
            + levels
                .iter()
                .map(|level| level.len() + std::mem::size_of::<u32>())
                .sum::<usize>();

        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0);

        header.write(&mut buffer[0..64]);

        let mut cur_index = 64;

        for level in levels {
            let write_end = level.len() as u32 / header.faces();
            let cur_end = cur_index + level.len() + std::mem::size_of::<u32>();

            if header.big_endian() {
                BigEndian::write_u32_into(
                    &[write_end],
                    &mut buffer[cur_index..cur_index + std::mem::size_of::<u32>()],
                );
            } else {
                LittleEndian::write_u32_into(
                    &[write_end],
                    &mut buffer[cur_index..cur_index + std::mem::size_of::<u32>()],
                )
            }

            buffer[cur_index + std::mem::size_of::<u32>()..cur_end].copy_from_slice(&level);

            cur_index = cur_end;
        }

        Ok(buffer)
    }
}
