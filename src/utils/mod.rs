use glium;
use time;

use std::vec::*;

use std::collections::BTreeMap;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

type TimeSample = (f64, usize);

type SampleHistory = Vec<f32>;

pub struct PerformaceCounters {
    samples: usize,
    acum_time: f64,
    times: BTreeMap<String, TimeSample>,
    digest: BTreeMap<String, SampleHistory>,
    digest_size: u32,
    digest_tick: u32,
}

impl PerformaceCounters {
    pub fn new() -> PerformaceCounters {
        PerformaceCounters {
            samples: 0,
            acum_time: 0.0,
            times: BTreeMap::new(),
            digest: BTreeMap::new(),
            digest_size: 50 as u32,
            digest_tick: 0 as u32,
        }
    }

    fn append(&mut self, delta: f64) {
        self.samples += 1;
        self.acum_time += delta;
    }
    fn get_fps(&self) -> f64 {
        self.samples as f64 / self.acum_time
    }
    fn reset(&mut self) {
        self.samples = 0;
        self.acum_time = 0 as f64;

        for e in self.times.iter_mut() {
            (e.1).0 = 0.0 as f64;
            (e.1).1 = 0 as usize;
        }
    }

    pub fn measure<F>(&mut self, name: &str, body: &mut F)
        where F: FnMut()
    {

        let start_t = time::precise_time_s();

        body();

        let end_t = time::precise_time_s();

        if let Some(x) = self.times.get_mut(name.into()) {
            x.0 += end_t - start_t;
            x.1 += 1;
            return;
        }

        self.times.insert(name.into(), (end_t - start_t, 1));
    }

    pub fn digest_measures(&mut self){

        let mut new_ones = BTreeMap::new();

        for (name, sample) in &self.times{
            let t = sample.0;
            let s = sample.1;

            let v =  t as f32 / s as f32;
            // normalize to 60fps
            let sixtyfps = 1.0 / 60,0;
            let v = v / sixtyfps;

            new_ones.insert(name, v);
        }

        for (name, new_measure) in new_ones{

            if let Some(x) = self.digest.get_mut(name.into()) {
                if x.len() < self.digest_size as usize{
                    x.resize(self.digest_size as usize, 0.0);
                }
                *(x.get_mut(self.digest_tick as usize).unwrap()) =  new_measure;
                continue;
            }
            
            let mut x = Vec::new();
            x.resize(self.digest_size as usize, 0.0);
            *(x.get_mut(self.digest_tick as usize).unwrap()) =  new_measure;

            self.digest.insert(name.clone(), x);
        }

        self.digest_tick = (self.digest_tick + 1) % self.digest_size;
    }

    pub fn get_last_measure(&self, name: &str) -> Option<f64> {
        if let Some(x) = self.times.get(name.into()) {
            Some(x.0 / x.1 as f64)
        } else {
            None
        }
    }

    pub fn get_measurements_for(&self, name: &str) -> Option<&[f32]>{

        match self.digest.get(name.into()){
            Some(x) => Some(x.as_slice()),
            None => None,
        }
    }

    // TODO: iterator to retrieve the digested 
}

/// infinite loop with iterations/second reporting every x seconds
/// it will pass delta time to function body
pub fn loop_with_report<'a, F: FnMut(f64, &mut PerformaceCounters)>(mut body: F, x: u32) {
    let mut pc = PerformaceCounters::new();
    if x == 0 {
        loop {
            body(0.0, &mut pc);
        }
    } else {
        loop {
            let mut delta: f64 = 0.0;

            let start = time::PreciseTime::now();
            while start.to(time::PreciseTime::now()) < time::Duration::seconds(x as i64) {
                let start_t = time::precise_time_s();

                body(delta, &mut pc);

                let end_t = time::precise_time_s();
                delta = end_t - start_t;
                pc.append(delta);
            }


            println!("fps: {} ", pc.get_fps());
            println!("prepass {:.6} ssao {:.6} blur {:.6} color {:.6}",
                     pc.get_last_measure("prepass").unwrap(),
                     pc.get_last_measure("ssao").unwrap(),
                     pc.get_last_measure("blur").unwrap(),
                     pc.get_last_measure("color").unwrap());


            pc.digest_measures();
            pc.reset();
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

        let axis_program = glium::Program::from_source(display,
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
    fn with_tess(&self) -> bool {
        self.axis_program.has_tessellation_shaders()
    }
}
