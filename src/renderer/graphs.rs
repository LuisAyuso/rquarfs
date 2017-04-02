extern crate glium;
use renderer::context;
use renderer::context::Program;
use renderer::shader::ProgramReloader;

use std::vec::*;
// use glium::texture;
// use glium::framebuffer::SimpleFrameBuffer;
// use glium::Surface;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Performace drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct PerfPos {
    position: (f32, f32),
}

pub struct GraphPlot<'a, 'b, D>
    where D: glium::Surface + 'a
{
    frame: &'a mut D,
    count: u32,
    program: &'b ProgramReloader,
}

implement_vertex!(PerfPos, position);

impl<'a, 'b, D> GraphPlot<'a, 'b, D>
    where D: glium::Surface
{
    pub fn new(ctx: &context::Context,
               frame: &'a mut D,
               tick: f32, // the current possition between 0 and 1
               program: &'b ProgramReloader)
               -> GraphPlot<'a, 'b, D> {

        let buffer =
            glium::VertexBuffer::new(ctx.display(),
                                     &[PerfPos { position: (-1.0, -0.9999) },
                                       PerfPos { position: (1.0, -0.9999) },

                                       PerfPos { position: (-1.0, 0.0) },
                                       PerfPos { position: (1.0, 0.0) },

                                       PerfPos { position: (-0.9999 + tick * 2.0, -1.0) },
                                       PerfPos { position: (-0.9999 + tick * 2.0, 1.0) },

                                       PerfPos { position: (-1.0, 0.9999) },
                                       PerfPos { position: (1.0, 0.9999) }])
                .unwrap();
        let uniforms = uniform!{
            color: get_color(0),
        };
        frame.draw(&buffer,
                  glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                  program.get_program(),
                  &uniforms,
                  &glium::DrawParameters {
                      viewport: Some(glium::Rect {
                          left: 660,
                          bottom: 10,
                          width: 1200,
                          height: 480,
                      }),
                      ..Default::default()
                  })
            .unwrap();

        GraphPlot {
            frame: frame,
            count: 1,
            program: program,
        }
    } // new

    // FIXME: do a build pattern here to print several slices.
    //  first call draws the axis in black.
    pub fn draw_values(&mut self, ctx: &context::Context, values: &[f32]) {
        let mut samples = Vec::new();
        samples.extend_from_slice(values);

        // horizontal scale
        let step = 2.0 / values.len() as f32;

        let verts: Vec<PerfPos> = values.iter()
            .enumerate()
            .map(|(x, y)| PerfPos { position: (-0.9999 + step * x as f32, -1.0 + *y * 2.0) })
            .collect();

        // plot axis lines
        let buffer = glium::VertexBuffer::new(ctx.display(), verts.as_slice()).unwrap();

        let uniforms = uniform!{
            color: get_color(self.count),
        };

        self.frame
            .draw(&buffer,
                  glium::index::NoIndices(glium::index::PrimitiveType::LineStrip),
                  self.program.get_program(),
                  &uniforms,
                  &glium::DrawParameters {
                      // backface_culling: glium::BackfaceCullingMode::CullClockwise,
                      viewport: Some(glium::Rect {
                          left: 660,
                          bottom: 10,
                          width: 1200,
                          height: 480,
                      }),
                      ..Default::default()
                  })
            .unwrap();

        self.count += 1;
    }

    pub fn finish(self) {
        // last function by value kills the borrow?  not doing the job
    }
}

fn get_color(i: u32) -> [f32; 4] {

    match i {   
        // R    G    B
        0 => [0.0, 0.0, 0.0, 1.0], // BLACK
        1 => [0.8, 0.0, 0.0, 1.0], // RED
        2 => [0.0, 0.8, 0.0, 1.0], // GREEN
        3 => [0.0, 0.0, 0.8, 1.0], // BLUE
        4 => [0.8, 0.0, 0.8, 1.0], // PURPLE
        5 => [0.8, 0.8, 0.8, 1.0], // GREY
        _ => [1.0, 1.0, 1.0, 1.0], // WHITE
    }
}
