extern crate glium;

use renderer::context;

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
    quad_program: glium::Program,
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

        let quad_program =
            glium::Program::from_source(ctx.display(),
                                        // vertex shader
                                        "
                #version 140
                in vec2 \
                                         position;
                in vec2 tex_coords;   

                \
                                         smooth out vec2 coords;

                void main() {
                    \
                                         gl_Position = vec4(position,0.0, 1.0); 
                    \
                                         coords = tex_coords;
                }
            ",
                                        // fragment shader
                                        "
                #version 140
                uniform \
                                         sampler2D quad_texture;
                smooth in vec2 \
                                         coords;
                out vec4 frag_color;

                \
                                         void main() {
                    frag_color = \
                                         texture(quad_texture, coords);
                    \
                                         //frag_color = vec4(coords, 0.0, 1.0);
                \
                                         }
            ",
                                        None)
                .unwrap();

        TexQuad {
            quad_program: quad_program,
            quad_buffer: quad_buffer.unwrap().into(),
        }
    } // new
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
        &self.quad_program
    }
}
