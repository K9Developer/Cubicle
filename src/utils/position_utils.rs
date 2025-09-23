use crate::constants::constants::{MCA_REGION_SECTOR_SIZE, REGION_CHUNK_LINE};
use crate::models::other::region::Region;
use crate::models::positions::chunk_position::ChunkPosition;
use crate::models::positions::whole_position::Position;
use crate::utils::generic_utils::div_rem_nonzero;

pub fn block_position_to_chunk_pos_and_block_index(pos: &Position, chunk_size: i32, min_y: i32) -> (ChunkPosition, usize) {
    let chunk_position = (pos.x().div_euclid(chunk_size), pos.z().div_euclid(chunk_size));
    let cpos = ChunkPosition::new(chunk_position.0, chunk_position.1, pos.dimension().clone());

    let local_x = pos.x().rem_euclid(16);
    let local_z = pos.z().rem_euclid(16);
    let local_y = pos.y() - min_y;
    let index = ((local_y * 16 + local_z) * 16 + local_x) as usize;

    (cpos, index)
}

pub fn world_position_to_chunk_position(x: f64, z: f64, chunk_size: i32) -> (i32, i32) {
    let chunk_x = (x.floor() as i32).div_euclid(chunk_size);
    let chunk_z = (z.floor() as i32).div_euclid(chunk_size);
    (chunk_x, chunk_z)
}

pub fn chunk_position_to_world_position(chunk_pos: (i32, i32), chunk_size: i32) -> (i32, i32) {
    (chunk_pos.0 * chunk_size, chunk_pos.1 * chunk_size)
}

pub fn block_index_to_block_position(chunk_pos: &ChunkPosition, index: usize, chunk_size: i32, min_y: i32) -> Position {
    let (local_y, left_over) = div_rem_nonzero(index as i32, chunk_size * chunk_size);
    let (local_z, local_x) = div_rem_nonzero(left_over, chunk_size);
    let actual_y = local_y + min_y;

    let (chunk_x, chunk_z) = chunk_position_to_world_position((chunk_pos.x(), chunk_pos.z()), chunk_size);

    (chunk_x + local_x, actual_y, chunk_z + local_z, chunk_pos.dimension().clone()).into()
}

pub fn is_position_within_bounding_box(position: &Position, corner1: &Position, corner2: &Position) -> bool {
    let (min_x, max_x) = (corner1.x().min(corner2.x()), corner1.x().max(corner2.x()));
    let (min_y, max_y) = (corner1.y().min(corner2.y()), corner1.y().max(corner2.y()));
    let (min_z, max_z) = (corner1.z().min(corner2.z()), corner1.z().max(corner2.z()));

    position.x() >= min_x && position.x() <= max_x &&
    position.y() >= min_y && position.y() <= max_y &&
    position.z() >= min_z && position.z() <= max_z
}

pub fn chunk_offset_to_position(offset: usize, region: &Region) -> (i32, i32) {
    let index = offset / 4;
    let local_x = (index % REGION_CHUNK_LINE) as i32;
    let local_z = (index / REGION_CHUNK_LINE) as i32;

    let chunk_x = region.position.x() * (REGION_CHUNK_LINE as i32) + local_x;
    let chunk_z = region.position.z() * (REGION_CHUNK_LINE as i32) + local_z;

    (chunk_x, chunk_z)
}

pub fn world_position_to_relative_chunk_position(x: i32, y: i32, z: i32, chunk_size: i32) -> (i32, i32, i32) {
    (
        x.rem_euclid(chunk_size),
        y.rem_euclid(chunk_size),
        z.rem_euclid(chunk_size),
    )
}

pub fn relative_position_to_world_position(chunk_pos: (i32, i32), relative_pos: (i32, i32, i32), chunk_size: i32) -> (i32, i32, i32) {
    let base = chunk_position_to_world_position(chunk_pos, chunk_size);
    (base.0 + relative_pos.0, relative_pos.1, base.1 + relative_pos.2)
}