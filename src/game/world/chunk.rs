use cgmath::{vec3, vec2};

use crate::graphics::{mesh::{Mesh, Vertex}, drawable::Drawable, utils::Side};

type BlockState = bool;
type ChunkData = Vec<BlockState>;
const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    mesh: Mesh,
    data: ChunkData,
}

impl Drawable for Chunk {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut data = vec![false; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        for i in 0 .. CHUNK_SIZE {
            for j in 0 .. CHUNK_SIZE / 2 {
                for k in 0 .. CHUNK_SIZE {
                    data[index_unchecked(i, j, k)] = true;
                }
            }
        }

        let vertices = mesh::culled(&data);
        let mesh = Mesh::new(device, vertices);

        return Self {
            mesh,
            data,
        };
    }
}

const fn block_face(side: Side, i: isize, j: isize, k: isize) -> [Vertex; 6] {
    return match side {
        Side::Top => [
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
        ],
        Side::Bottom => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
        ],
        Side::Right => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
        ],
        Side::Left => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
        ],
        Side::Front => [
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
        ],
        Side::Back => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
        ],
    };
}

const fn index(x: isize, y: isize, z: isize) -> usize {
    const MAX: isize = CHUNK_SIZE as isize;
    const VOL: isize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as isize;
    return if x < 0 || y < 0 || z < 0 || x >= MAX || y >= MAX || z >= MAX { VOL as usize }
    else { ((z * MAX * MAX) + (y * MAX) + x) as usize };
}

const fn index_unchecked(x: usize, y: usize, z: usize) -> usize {
    return (z * CHUNK_SIZE * CHUNK_SIZE) + (y * CHUNK_SIZE) + x;
}

mod mesh {
    use crate::{graphics::{mesh::Vertex, utils::Side}, game::world::chunk::index};
    use super::{ChunkData, CHUNK_SIZE, block_face};

    // Meshing algorithms

    // Creates 6 faces for each voxel
    pub fn simple(data: &ChunkData) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    if data[index(i, j, k)] {
                        vertices.extend(block_face(Side::Top,    i, j, k));
                        vertices.extend(block_face(Side::Bottom, i, j, k));
                        vertices.extend(block_face(Side::Right,  i, j, k));
                        vertices.extend(block_face(Side::Left,   i, j, k));
                        vertices.extend(block_face(Side::Front,  i, j, k));
                        vertices.extend(block_face(Side::Back,   i, j, k));
                    }
                }

            }

        }
        
        return vertices;
    }

    // Creates only the faces visible from outside
    pub fn culled(data: &ChunkData) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    if data[index(i, j, k)] {
                        const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
                        let v = index(i, j + 1, k); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Top,    i, j, k)); } } else { vertices.extend(block_face(Side::Top,    i, j, k)); }
                        let v = index(i, j - 1, k); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Bottom, i, j, k)); } } else { vertices.extend(block_face(Side::Bottom, i, j, k)); }
                        let v = index(i, j, k + 1); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Right,  i, j, k)); } } else { vertices.extend(block_face(Side::Right,  i, j, k)); }
                        let v = index(i, j, k - 1); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Left,   i, j, k)); } } else { vertices.extend(block_face(Side::Left,   i, j, k)); }
                        let v = index(i + 1, j, k); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Front,  i, j, k)); } } else { vertices.extend(block_face(Side::Front,  i, j, k)); }
                        let v = index(i - 1, j, k); if v != CHUNK_VOLUME { if unsafe { !data.get_unchecked(v) } { vertices.extend(block_face(Side::Back,   i, j, k)); } } else { vertices.extend(block_face(Side::Back,   i, j, k)); }
                    }
                }

            }

        }
        
        return vertices;
    }
}