extern crate glium;
use renderer::context;
use renderer::context::Program;
use renderer::shader::ProgramReloader;

use std::vec::*;
//use glium::texture;
//use glium::framebuffer::SimpleFrameBuffer;
//use glium::Surface;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Performace drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct PerfPos {
    position: (f32, f32),
}

pub struct GraphPlot {
    program: ProgramReloader,
}

implement_vertex!(PerfPos, position);

impl GraphPlot {

    pub fn new(ctx: &context::Context) -> GraphPlot {

        let program = ProgramReloader::new(ctx.display(), "performance").unwrap();

        GraphPlot {
            program: program,
        }
    } // new

    // FIXME: do a build pattern here to print several slices.
    //  first call draws the axis in black.
    pub fn draw_values<D>(&mut self, ctx :&context::Context, frame: &mut D, values: &[f32]) 
        where D : glium::Surface
    {
        let mut samples = Vec::new();
        samples.extend_from_slice(values);
    
        // horizontal scale
        let step = 2.0 / values.len() as f32;

        let verts : Vec<PerfPos> = values.iter().enumerate().map(|(x, y)|{ 
                                     PerfPos{position:( -1.0 + step * x as f32, -1.0 + *y * 2.0 )} } ).collect(); 

        // plot axis lines
        let buffer = glium::VertexBuffer::new(ctx.display(), verts.as_slice()).unwrap();

        let uniforms = uniform!{};

        frame.draw(&buffer,
                  glium::index::NoIndices(glium::index::PrimitiveType::LineStrip),
                  self.program.get_program(),
                  &uniforms,
                  &glium::DrawParameters {
                                      // backface_culling: glium::BackfaceCullingMode::CullClockwise,
                                      viewport: Some(glium::Rect {
                                          left: 660,
                                          bottom: 10,
                                          width: 1400,
                                          height: 480,
                                      }),
                                      ..Default::default()
                  })

            .unwrap();
    }
}
