fn main() {
    let future = voxelgame::run();
    pollster::block_on(future);
}