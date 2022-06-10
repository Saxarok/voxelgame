pub trait Bindable {
    fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, index: u32);
    fn layout(&self) -> &wgpu::BindGroupLayout;
}