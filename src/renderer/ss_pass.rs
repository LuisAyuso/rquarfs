extern crate glium;

use renderer::context;
use renderer::context::Program;
use renderer::context::Context;
use renderer::shader::ProgramReloader;
use glium::texture;
use glium::Surface;
use cgmath::Matrix4;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Quad with texture drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct QuadVert {
    position: (f32, f32),
    tex_coords: (f32, f32),
}

pub struct ScreenSpacePass<'a> {
    quad_buffer: glium::vertex::VertexBufferAny,
    program: ProgramReloader,
    fb: glium::framebuffer::SimpleFrameBuffer<'a>,
    size: (u32, u32),
}


implement_vertex!(QuadVert, position, tex_coords);

impl<'a> ScreenSpacePass<'a> {
    pub fn new(ctx: &context::Context,
               program: &str,
               output_texture: &'a glium::texture::Texture2d,
               depth_buffer: &'a texture::DepthTexture2d)
               -> ScreenSpacePass<'a> {

        let quad_buffer = glium::VertexBuffer::new(ctx.display(),
                                                   &[QuadVert {
                                                         position: (-1.0, -1.0),
                                                         tex_coords: (0.0, 0.0),
                                                     },
                                                     QuadVert {
                                                         position: (-1.0, 1.0),
                                                         tex_coords: (0.0, 1.0),
                                                     },
                                                     QuadVert {
                                                         position: (1.0, -1.0),
                                                         tex_coords: (1.0, 0.0),
                                                     },
                                                     QuadVert {
                                                         position: (1.0, 1.0),
                                                         tex_coords: (1.0, 0.0),
                                                     }]);


        let mut frame = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(),
                                                                                 output_texture,
                                                                                 depth_buffer)
            .unwrap();
        frame.clear_depth(1.0);
        let program = ProgramReloader::new(ctx, program).unwrap();
        let dim = frame.get_dimensions();

        ScreenSpacePass {
            quad_buffer: quad_buffer.unwrap().into(),
            program: program,
            fb: frame,
            size: dim,
        }
    } // new

    pub fn update(&mut self, ctx: &Context, delta: f64) {
        self.program.update(ctx, delta);
    }

    pub fn execute_pass(&mut self,
                        inverse_matrix: &Matrix4<f32>,
                        input_texture: &glium::texture::Texture2d,
                        depth_texture: &texture::DepthTexture2d,
                        noise_texture: &glium::texture::Texture2d) {
        let uniforms = uniform! {
                input_texture: input_texture,
                depth_texture: depth_texture,
                noise_texture: noise_texture,
                inversei_matrix: Into::<[[f32; 4]; 4]>::into(*inverse_matrix),
                frame_size: self.size,
        };

        let parameters = glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
            depth: glium::Depth {
                test: glium::DepthTest::IfLessOrEqual,
                write: false,
                ..Default::default()
            },
            polygon_mode: glium::PolygonMode::Fill,
            provoking_vertex: glium::draw_parameters::ProvokingVertex::LastVertex,
            ..Default::default()
        };

        self.fb
            .draw(&self.quad_buffer,
                  glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                  self.program.get_program(),
                  &uniforms,
                  &parameters)
            .unwrap();


    }
}
