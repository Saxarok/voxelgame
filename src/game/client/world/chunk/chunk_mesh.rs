use cgmath::{vec3, vec2};
use euclid::Box2D;

use crate::graphics::{mesh::{Vertex, Mesh}, utils::Side, drawable::Drawable, atlas::Atlas};

use super::chunk::{BlockState, CHUNK_SIZE};

pub struct ChunkMesh {
    mesh: Mesh
}

impl ChunkMesh {
    pub fn new(device: &wgpu::Device, blocks: &[BlockState], texture_atlas: &Atlas<BlockState>) -> Self {
        let vertices = mesh::culled::<CHUNK_SIZE>(blocks, texture_atlas);
        let mesh = Mesh::new(device, vertices);

        return Self {
            mesh
        };
    }

    pub fn rebuild(&mut self, device: &wgpu::Device, blocks: &[BlockState], texture_atlas: &Atlas<BlockState>) {
        self.mesh.vertices = mesh::culled::<CHUNK_SIZE>(blocks, texture_atlas);
        self.mesh.bake(device);
    }
}

impl Drawable for ChunkMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

const fn block_face(side: Side, i: isize, j: isize, k: isize, uv: Box2D<f32, f32>) -> [Vertex; 6] {
    let (ox, oy) = (uv.min.x, uv.min.y);
    let (sx, sy) = (uv.max.x, uv.max.y);
    return match side {
        Side::Top => [
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
        ],
        Side::Bottom => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
        ],
        Side::Right => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(ox, oy) },
        ],
        Side::Left => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, sy) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(sx, oy) },
        ],
        Side::Front => [
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(ox, sy) },
            
            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(sx, oy) },
        ],
        Side::Back => [
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(sx, oy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(sx, sy) },
            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(ox, sy) },
            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(ox, oy) },
        ],
    };
}

#[allow(dead_code)]
mod mesh {
    use super::*;

    // Meshing algorithms
    // Creates 6 faces for each voxel
    pub fn simple<const CHUNK_SIZE: usize>(data: &[BlockState], texture_atlas: &Atlas<BlockState>) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    let block_state = data[index::<CHUNK_SIZE>(i, j, k)];
                    if block_state != BlockState::AIR {
                        let uv = texture_atlas.uv(&block_state);
                        vertices.extend(block_face(Side::Top,    i, j, k, uv));
                        vertices.extend(block_face(Side::Bottom, i, j, k, uv));
                        vertices.extend(block_face(Side::Right,  i, j, k, uv));
                        vertices.extend(block_face(Side::Left,   i, j, k, uv));
                        vertices.extend(block_face(Side::Front,  i, j, k, uv));
                        vertices.extend(block_face(Side::Back,   i, j, k, uv));
                    }
                }

            }

        }
        
        return vertices;
    }

    // Creates only the faces visible from outside
    pub fn culled<const CHUNK_SIZE: usize>(data: &[BlockState], texture_atlas: &Atlas<BlockState>) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE as isize {
            for j in 0 .. CHUNK_SIZE as isize {
                for k in 0 .. CHUNK_SIZE as isize {
                    let block_state = data[index::<CHUNK_SIZE>(i, j, k)];
                    if block_state != BlockState::AIR {
                        let uv = texture_atlas.uv(&block_state);
                        let chunk_volume: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
                        let v = index::<CHUNK_SIZE>(i, j + 1, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Top,    i, j, k, uv)); } } else { vertices.extend(block_face(Side::Top,    i, j, k, uv)); }
                        let v = index::<CHUNK_SIZE>(i, j - 1, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Bottom, i, j, k, uv)); } } else { vertices.extend(block_face(Side::Bottom, i, j, k, uv)); }
                        let v = index::<CHUNK_SIZE>(i, j, k + 1); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Right,  i, j, k, uv)); } } else { vertices.extend(block_face(Side::Right,  i, j, k, uv)); }
                        let v = index::<CHUNK_SIZE>(i, j, k - 1); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Left,   i, j, k, uv)); } } else { vertices.extend(block_face(Side::Left,   i, j, k, uv)); }
                        let v = index::<CHUNK_SIZE>(i + 1, j, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Front,  i, j, k, uv)); } } else { vertices.extend(block_face(Side::Front,  i, j, k, uv)); }
                        let v = index::<CHUNK_SIZE>(i - 1, j, k); if v != chunk_volume { if unsafe { *data.get_unchecked(v) == BlockState::AIR } { vertices.extend(block_face(Side::Back,   i, j, k, uv)); } } else { vertices.extend(block_face(Side::Back,   i, j, k, uv)); }
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