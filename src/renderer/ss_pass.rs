extern crate glium;

use renderer::context;
use renderer::context::Program;
use renderer::shader::ProgramReloader;
use glium::texture;
use glium::Surface;

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
}


implement_vertex!(QuadVert, position, tex_coords);

impl<'a> ScreenSpacePass<'a> {
    pub fn new(ctx: &context::Context, 
               program: &str,
               texture: &'a glium::texture::Texture2d,
               depth_buffer: &'a texture::DepthTexture2d,
               ) -> ScreenSpacePass<'a> {

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
                                                         position: (1.0, -1.0),
                                                         tex_coords: (1.0, 0.0),
                                                     },
                                                     QuadVert {
                                                         position: (1.0, 1.0),
                                                         tex_coords: (1.0, 1.0),
                                                     },
                                                     QuadVert {
                                                         position: (-1.0, 1.0),
                                                         tex_coords: (0.0, 1.0),
                                                     }]);



        let frame =  glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(), 
                                                                         texture, 
                                                                         depth_buffer).unwrap();

        let program = ProgramReloader::new(ctx.display(),program).unwrap();

        ScreenSpacePass {
            quad_buffer: quad_buffer.unwrap().into(),
            program: program,
            fb: frame,
        }
    } // new

    pub fn update<F: glium::backend::Facade>(&mut self, display: &F, delta: f64){
        self.program.update(display, delta);
    }

    pub fn execute_pass(&mut self, input_texture: &'a glium::texture::Texture2d){
        let uniforms = uniform! {
                input_texture: input_texture 
        };

        let parameters =  glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            depth: glium::Depth {
                test: glium::DepthTest::Ignore,
                write: false,
                ..Default::default()
            },
            polygon_mode: glium::PolygonMode::Fill, 
            provoking_vertex: glium::draw_parameters::ProvokingVertex::LastVertex,
            ..Default::default()
        };

        self.fb
            .draw(&self.quad_buffer,
                  glium::index::NoIndices(  glium::index::PrimitiveType::TrianglesList),
                  self.program.get_program(),
                  &uniforms,
                  &parameters).unwrap();


    }
}

