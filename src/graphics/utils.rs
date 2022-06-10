pub mod Utils {
    pub fn render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
        return encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                // [[location(0)]] in the fragment shader
                wgpu::RenderPassColorAttachment {
                    resolve_target: None,
                    view: view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(
                            wgpu::Color {
                                r: 0.01,
                                g: 0.01,
                                b: 0.01,
                                a: 1.0,
                            }
                        ),
                        store: true,
                    }
                }
            ],
            depth_stencil_attachment: None,
        });
    }

    pub fn render<'a>(encoder : &'a mut wgpu::CommandEncoder,
                      view    : &'a wgpu::TextureView,
                      lambda  : impl FnOnce(wgpu::RenderPass<'a>) -> ()) {
        let pass = render_pass(encoder, view);
        lambda(pass);
    }

    pub fn submit(queue: &wgpu::Queue, device: &wgpu::Device, lambda: impl FnOnce(&mut wgpu::CommandEncoder)) {
        let descriptor = wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") };
        let mut encoder = device.create_command_encoder(&descriptor);
        lambda(&mut encoder);

        queue.submit(std::iter::once(encoder.finish()));
    }
}