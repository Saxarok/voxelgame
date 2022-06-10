use wgpu::Device;

use super::mesh::Vertex;

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

pub fn pipeline(device: &Device,
                shader: &wgpu::ShaderModule,
                config: &wgpu::SurfaceConfiguration,
                groups: &[&wgpu::BindGroupLayout]) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        push_constant_ranges : &[],
        bind_group_layouts   : groups,
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label  : Some("Render Pipeline"),
        layout : Some(&pipeline_layout),

        // Shaders
        vertex: wgpu::VertexState {
            module      : &shader,
            entry_point : "vertex_main",
            buffers     : &[
                Vertex::describe(),
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module      : &shader,
            entry_point : "fragment_main",
            targets     : &[wgpu::ColorTargetState {
                format     : config.format,
                blend      : Some(wgpu::BlendState::REPLACE),
                write_mask : wgpu::ColorWrites::ALL,
            }],
        }),

        // Other
        primitive: wgpu::PrimitiveState {
            topology           : wgpu::PrimitiveTopology::TriangleList,
            strip_index_format : None,
            front_face         : wgpu::FrontFace::Ccw,
            cull_mode          : Some(wgpu::Face::Back),
            polygon_mode       : wgpu::PolygonMode::Fill, // Other modes require Features::NON_FILL_POLYGON_MODE
            unclipped_depth    : false,                   // Requires Features::DEPTH_CLIP_CONTROL
            conservative       : false,                   // Requires Features::CONSERVATIVE_RASTERIZATION
        },
        depth_stencil : None,
        multiview     : None,
        multisample   : wgpu::MultisampleState {
            count                     : 1,
            mask                      : !0,
            alpha_to_coverage_enabled : false,
        },
    });
}