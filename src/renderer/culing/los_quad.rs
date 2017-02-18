
// line of sight map, preview

extern crate glium;

use renderer::context;
use super::los::Los;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    los with texture drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct LosVert {
    position: (f32, f32),
}

pub struct LosQuad {
    los_program: glium::Program,
}


implement_vertex!(LosVert, position);

impl LosQuad {
    pub fn new(ctx: &context::Context) -> LosQuad {

        let los_program =
            glium::Program::from_source(ctx.display(),
                                        // vertex shader
                                        "
                #version 140
                in vec2 \
                                         position;
                void main() {
                    \
                                         gl_Position = vec4(position.x, position.y, 0.0, 1.0); 
                \
                                         }
            ",
                                        // fragment shader
                                        "
                #version 140
                out vec4 \
                                         frag_color;
                void main() {
                    \
                                         frag_color = vec4( 0.8, 0.4, 0.4, 1.0);
                \
                                         }
            ",
                                        None)
                .unwrap();

        LosQuad { los_program: los_program }
    } // new

    pub fn get_drawable<'s>(&'s self, ctx: &context::Context, los: &Los) -> LosQuadDraw<'s> {

        // generate vertices out of patches!
        // yes, i know this should be cached or something, but for now it will make the job

        let (w, h) = los.dimensions();
        let patches = los.get_patches();

        let mut vertices: Vec<LosVert> = Vec::new();
        for tmp in patches.iter() {

            let mut point: (f32, f32) = (tmp.p.0 as f32, tmp.p.1 as f32);
            let mut vector: (f32, f32) = (tmp.v.0 as f32, tmp.v.1 as f32);

            point.0 = point.0 / w as f32 * 2.0 - 1.0;
            point.1 = point.1 / w as f32 * 2.0 - 1.0;
            vector.0 = vector.0 / h as f32 * 2.0;
            vector.1 = vector.1 / h as f32 * 2.0;

            vertices.push(LosVert { position: (point.0, point.1) });
            vertices.push(LosVert { position: (point.0, (point.1 + vector.1)) });
            vertices.push(LosVert { position: (point.0, (point.1 + vector.1)) });
            vertices.push(LosVert { position: ((point.0 + vector.0), (point.1 + vector.1)) });
            vertices.push(LosVert { position: ((point.0 + vector.0), (point.1 + vector.1)) });
            vertices.push(LosVert { position: ((point.0 + vector.0), point.1) });
            vertices.push(LosVert { position: ((point.0 + vector.0), point.1) });
            vertices.push(LosVert { position: (point.0, point.1) });
        }

        let vert_buffer = glium::VertexBuffer::new(ctx.display(), vertices.as_slice()).unwrap();
        LosQuadDraw::<'s> {
            quad: self,
            vert_buffer: vert_buffer.into(),
        }
    }
}

pub struct LosQuadDraw<'a> {
    quad: &'a LosQuad,
    vert_buffer: glium::vertex::VertexBufferAny,
}


use renderer::context::{DrawItem, Program};

impl<'a> DrawItem for LosQuadDraw<'a> {
    fn get_vertices(&self) -> &glium::vertex::VertexBufferAny {
        &self.vert_buffer
    }
    fn get_primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::LinesList
    }
}

impl<'a> Program for LosQuadDraw<'a> {
    fn get_program(&self) -> &glium::Program {
        &self.quad.los_program
    }
    fn with_tess(&self) -> bool {
        self.quad.los_program.has_tessellation_shaders()
    }
}
