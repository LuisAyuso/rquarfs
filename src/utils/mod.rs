use glium;
use time;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct PerformaceCounters{
    samples: usize,
    acum_time:  f64,
}

impl PerformaceCounters{
    fn new() -> PerformaceCounters{
        PerformaceCounters{
            samples: 0,
            acum_time: 0.0
        }
    }

    fn append(&mut self, delta: f64){
        self.samples+=1;
        self.acum_time+=delta;
    }
    fn get_fps(&self) -> f64{
         self.samples as f64 / self.acum_time
    }
    fn reset(&mut self){
        self.samples = 0;
        self.acum_time = 0 as f64;
    }
}

/// infinite loop with iterations/second reporting every x seconds
/// it will pass delta time to function body
pub fn loop_with_report<F: FnMut(f64)>(mut body: F, x: u32) {
    let mut p = PerformaceCounters::new();
    if x == 0 {
        loop {
            body(0.0);
        }
    } else {
        loop {
            let mut delta: f64 = 0.0;
            p.reset();
            
            let start = time::PreciseTime::now();
            while start.to(time::PreciseTime::now()) < time::Duration::seconds(x as i64) {
                let start_t = time::precise_time_s();

                body(delta);

                let end_t = time::precise_time_s();
                delta = end_t - start_t;
                p.append(delta);
            }

            println!("fps: {} ", p.get_fps());
        }
    }
}
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Axis drawing
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct AxisVert {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
}

pub struct Axis {
    axis_buffer: glium::vertex::VertexBufferAny,
    axis_program: glium::Program,
}


implement_vertex!(AxisVert, position, color);

impl Axis {
    pub fn new<F: glium::backend::Facade>(display: &F) -> Axis {

        // plot axis lines
        let axis_buffer = glium::VertexBuffer::new(display,
                                                   &[AxisVert {
                                                         position: (0.0, 0.0, 0.0),
                                                         color: (1.0, 0.0, 0.0),
                                                     },
                                                     AxisVert {
                                                         position: (100.0, 0.0, 0.0),
                                                         color: (1.0, 0.0, 0.0),
                                                     },
                                                     AxisVert {
                                                         position: (0.0, 0.0, 0.0),
                                                         color: (0.0, 1.0, 0.0),
                                                     },
                                                     AxisVert {
                                                         position: (0.0, 100.0, 0.0),
                                                         color: (0.0, 1.0, 0.0),
                                                     },
                                                     AxisVert {
                                                         position: (0.0, 0.0, 0.0),
                                                         color: (0.0, 0.0, 1.0),
                                                     },
                                                     AxisVert {
                                                         position: (0.0, 0.0, 100.0),
                                                         color: (0.0, 0.0, 1.0),
                                                     }])
            .unwrap()
            .into();

        let axis_program =
            glium::Program::from_source(display,
                                        // vertex shader
             r#"
                 #version 140
                 in vec3 position;
                 in vec3 color;
                 uniform mat4 perspective;
                 uniform mat4 view;

                out vec3 f_color;

                void main() {
                    gl_Position = perspective * view * vec4(position, 1.0);
                    f_color = color;
                }
            "#,
                                        // fragment shader
            r#"
                #version 140
                in vec3  f_color;
                out vec4 color; 
                void main() {
                    color = vec4(f_color, 1.0);
                }
            "#,
                                        None)
                .unwrap();

        Axis {
            axis_program: axis_program,
            axis_buffer: axis_buffer,
        }
    } // new
}

use renderer::context::{DrawItem, Program};

impl DrawItem for Axis {
    fn get_vertices(&self) -> &glium::vertex::VertexBufferAny {
        &self.axis_buffer
    }
    fn get_primitive(&self) -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::LinesList
    }
}

impl Program for Axis {
    fn get_program(&self) -> &glium::Program {
        &self.axis_program
    }
    fn with_tess(&self) -> bool{
        self.axis_program.has_tessellation_shaders()
    }
}
