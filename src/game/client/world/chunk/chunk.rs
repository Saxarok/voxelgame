#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlockState {
    AIR,
    TEST,
    PANEL,
}

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    pub blocks: Vec<BlockState>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut data = vec![BlockState::AIR; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        for i in 0 .. CHUNK_SIZE {
            for j in 0 .. CHUNK_SIZE / 4 {
                for k in 0 .. CHUNK_SIZE {
                    if k % 2 == 0 {
                        data[Self::index_unchecked(i, j, k)] = BlockState::TEST;
                    } else {    
                        data[Self::index_unchecked(i, j, k)] = BlockState::PANEL;
                    }

                }
            }
        }

        return Self {
            blocks: data,
        };
    }

    const fn index_unchecked(x: usize, y: usize, z: usize) -> usize {
        return (z * CHUNK_SIZE * CHUNK_SIZE) + (y * CHUNK_SIZE) + x;
    }
}
