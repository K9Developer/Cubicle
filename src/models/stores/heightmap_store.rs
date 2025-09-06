use std::sync::Arc;
use crate::constants::versions::Version;
use crate::types::HeightmapKind;
use crate::utils::position_utils::world_position_to_relative_chunk_position;

const HEIGHTMAP_COUNT: usize = 4;

impl HeightmapKind {
    fn as_index(self) -> usize {
        self as usize
    }
}

#[derive(Debug)]
pub struct Heightmap {
    distances_from_bottom: Vec<i64>, // x fast, z slow
    lowest_y: i64,
    chunk_size: i32
}

impl Heightmap {
    pub fn new(version: &Arc<Version>) -> Self {
        Heightmap {
            distances_from_bottom: vec![0; (version.data.chunk_size * version.data.chunk_size) as usize],
            lowest_y: version.data.lowest_y as i64,
            chunk_size: version.data.chunk_size,
        }
    }

    pub unsafe fn set_via_longs(&mut self, longs: Vec<i64>) {
        let bits_per_entry = (longs.len() as u32 * u64::BITS) / (self.chunk_size * self.chunk_size) as u32;
        let entries_per_long = (u64::BITS / bits_per_entry) as usize;
        let max_entries: usize = (self.chunk_size * self.chunk_size) as usize;
        let mask: i64 = (1 << bits_per_entry) - 1;

        let mut current_entry_count: usize = 0;
        for long in longs {
            let mut shifted_value = long;

            for _ in 0..entries_per_long {
                if current_entry_count == max_entries { return; }
                let height_from_bottom = shifted_value & mask;
                *self.distances_from_bottom.get_unchecked_mut(current_entry_count) = height_from_bottom;
                shifted_value >>= bits_per_entry;
                current_entry_count += 1;
            }
        }
    }

    pub fn get_highest_y_at_position(&self, x: i32, z: i32) -> i32 {
        let (x, _, z) = world_position_to_relative_chunk_position(x, 0, z, self.chunk_size);
        let index = (z * self.chunk_size + x) as usize;
        (self.distances_from_bottom[index] + self.lowest_y - 1) as i32
    }
}

#[derive(Debug)]
pub struct HeightmapStore {
    heightmap_map: [Heightmap; HEIGHTMAP_COUNT],
    version: Arc<Version>,
}

impl HeightmapStore {
    pub fn new(version: Arc<Version>) -> Self {
        HeightmapStore {
            heightmap_map: std::array::from_fn(|_| Heightmap::new(&version)),
            version
        }
    }

    pub fn get_kind(&self, kind: HeightmapKind) -> &Heightmap {
        &self.heightmap_map[kind.as_index()]
    }

    pub fn get_kind_mut(&mut self, kind: HeightmapKind) -> &mut Heightmap {
        &mut self.heightmap_map[kind.as_index()]
    }
}