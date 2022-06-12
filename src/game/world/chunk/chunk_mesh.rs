use cgmath::{vec3, vec2};

use crate::graphics::{mesh::{Vertex, Mesh}, utils::Side, drawable::Drawable};

use super::chunk::{BlockState, CHUNK_SIZE};

pub struct ChunkMesh {
    mesh: Mesh
}

impl ChunkMesh {
    pub fn new(device: &wgpu::Device, blocks: &[BlockState]) -> Self {
        let vertices = mesh::culled::<CHUNK_SIZE>(blocks);
        let mesh = Mesh::new(device, vertices);

        return Self {
            mesh
        };
    }
}

impl Drawable for ChunkMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
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

#[allow(dead_code)]
mod mesh {
    use super::*;

    // Meshing algorithms
    // Creates 6 faces for each voxel
    pub fn simple<const CHUNK_SIZE: usize>(data: &[BlockState]) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    if data[index::<CHUNK_SIZE>(i, j, k)] != BlockState::AIR {
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
    pub fn culled<const CHUNK_SIZE: usize>(data: &[BlockState]) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    if data[index::<CHUNK_SIZE>(i, j, k)] != BlockState::AIR {
                        let chunk_volume: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
                        let v = index::<CHUNK_SIZE>(i, j + 1, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Top,    i, j, k)); } } else { vertices.extend(block_face(Side::Top,    i, j, k)); }
                        let v = index::<CHUNK_SIZE>(i, j - 1, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Bottom, i, j, k)); } } else { vertices.extend(block_face(Side::Bottom, i, j, k)); }
                        let v = index::<CHUNK_SIZE>(i, j, k + 1); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Right,  i, j, k)); } } else { vertices.extend(block_face(Side::Right,  i, j, k)); }
                        let v = index::<CHUNK_SIZE>(i, j, k - 1); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Left,   i, j, k)); } } else { vertices.extend(block_face(Side::Left,   i, j, k)); }
                        let v = index::<CHUNK_SIZE>(i + 1, j, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Front,  i, j, k)); } } else { vertices.extend(block_face(Side::Front,  i, j, k)); }
                        let v = index::<CHUNK_SIZE>(i - 1, j, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Back,   i, j, k)); } } else { vertices.extend(block_face(Side::Back,   i, j, k)); }
                    }
                }

            }

        }
        
        return vertices;
    }

    const fn index<const CHUNK_SIZE: usize>(x: isize, y: isize, z: isize) -> usize {
        let chunk_size = CHUNK_SIZE as isize;
        let chunk_volume = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as isize;
        return if x < 0 || y < 0 || z < 0 || x >= chunk_size || y >= chunk_size || z >= chunk_size { chunk_volume as usize }
        else { ((z * chunk_size * chunk_size) + (y * chunk_size) + x) as usize };
    }
}