extern crate glium;

use renderer::context;
use renderer::shader::ProgramReloader;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Quad with texture drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct QuadVert {
    position: (f32, f32),
    tex_coords: (f32, f32),
}

pub struct TexQuad {
    quad_buffer: glium::vertex::VertexBufferAny,
    quad_program: ProgramReloader,
}


implement_vertex!(QuadVert, position, tex_coords);

impl TexQuad {
    pub fn new(ctx: &context::Context) -> TexQuad {

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

        let program = ProgramReloader::new(ctx.display(),"tex_quad").unwrap();

        TexQuad {
            quad_program: program,
            quad_buffer: quad_buffer.unwrap().into(),
        }
    } // new

    pub fn update<F: glium::backend::Facade>(&mut self, display: &F, delta: f64){
        self.quad_program.update(display, delta);
    }
}

use renderer::context::{DrawItem, Program};

impl DrawItem for TexQuad {
    fn get_vertices(&self) -> &glium::vertex::VertexBufferAny {
        &self.quad_buffer
    }
    fn get_primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::TrianglesList
    }
}

impl Program for TexQuad {
    fn get_program(&self) -> &glium::Program {
        self.quad_program.get_program()
    }
    fn with_tess(&self) -> bool{
        self.quad_program.get_program().has_tessellation_shaders()
    }
}
