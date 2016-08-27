
// line of sight map, preview

extern crate glium;

use renderer::context;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Lot with texture drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct LotVert {
    position: (f32, f32),
    tex_coords: (f32, f32),
}

pub struct TexLot {
    lot_buffer:glium::vertex::VertexBufferAny,
    lot_program: glium::Program,
}


implement_vertex!(LotVert, position, tex_coords);

impl TexLot {

    pub fn new(ctx: &context::Context) -> TexLot {

        let lot_buffer = glium::VertexBuffer::new(ctx.display(),
                  &[LotVert { position: (-1.0,-1.0), tex_coords: (0.0, 0.0), },
                    LotVert { position: (-1.0, 1.0), tex_coords: (0.0, 1.0), },
                    LotVert { position: ( 1.0,-1.0), tex_coords: (1.0, 0.0), },

                    LotVert { position: ( 1.0,-1.0), tex_coords: (1.0, 0.0), },
                    LotVert { position: ( 1.0, 1.0), tex_coords: (1.0, 1.0), },
                    LotVert { position: (-1.0, 1.0), tex_coords: (0.0, 1.0), },
                ]
        );

        let lot_program =
            glium::Program::from_source(ctx.display(),
                // vertex shader
            "
                #version 140
                in vec2 position;
                in vec2 tex_coords;   

                smooth out vec2 coords;

                void main() {
                    gl_Position = vec4(position,0.0, 1.0); 
                    coords = tex_coords;
                }
            ",
               // fragment shader
            "
                #version 140
                uniform sampler2D lot_texture;
                smooth in vec2 coords;
                out vec4 frag_color;

                void main() {
                    frag_color = ve4( 0.5, 0.5, 0.5, 1.0);
                }
            ", None).unwrap();

        TexLot {
            lot_program: lot_program,
            lot_buffer: lot_buffer.unwrap().into(),
        }
    } // new
    
    pub fn get_program(& self) -> &glium::program::Program{
        &self.lot_program
    }

    pub fn get_vertices (& self)-> &context::VerticesT{
         &self.lot_buffer
    }
    pub fn get_primitive(&self) -> glium::index::PrimitiveType{        
        glium::index::PrimitiveType::TrianglesList
    }

}


