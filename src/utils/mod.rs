use glium;
use time;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// infinite loop with iterations/second reporting every x seconds
/// it will pass delta time to function body
pub fn loop_with_report<F: FnMut(f64)>(mut body: F, x: u32) {
    if x == 0 {
        loop {
            body(0.0);
        }
    } else {
        loop {
            let mut fps_accum: f64 = 0.0;
            let mut samples: u32 = 0;
            let mut delta: f64 = 0.0;

            let start = time::PreciseTime::now();
            while start.to(time::PreciseTime::now()) < time::Duration::seconds(x as i64) {
                let start_t = time::precise_time_s();

                body(delta);

                let end_t = time::precise_time_s();
                delta = end_t - start_t;
                fps_accum += delta;
                samples += 1;
            }

            print!("fps: {} \n", (samples as f64) / fps_accum);
        }
    }
}
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    convert to vertex + index
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


pub fn index_vertex_list<T> (vertices_org: &[T]) -> (Vec<T>, Vec<u32>)
where T: PartialEq  + Clone
{

    let mut vertices : Vec<T> = Vec::new();
    let mut indices : Vec<u32> = Vec::new();

    // for each vertex, search in vertices list, if not there, insert the last one.
    for v in vertices_org{
        let pos = vertices.iter().position(|r| *r == *v);
        match pos{
            None => { vertices.push(v.clone()); indices.push((vertices.len() - 1) as u32); },
            Some(x) => indices.push(x as u32),
        }
    }
    vertices.shrink_to_fit();
    indices.shrink_to_fit();

    (vertices, indices)
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
    axis_buffer:glium::vertex::VertexBufferAny,
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
            .unwrap().into();

        let axis_program =
            glium::Program::from_source(display,
                                        // vertex shader
                                        "
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
            ",
                                        // fragment shader
                                        "
                #version 140
                in vec3  f_color;
                out vec4 color; 
                void main() {
                    color = vec4(f_color, 1.0);
                }
            ",
                                        None)
                .unwrap();

        Axis {
            axis_program: axis_program,
            axis_buffer: axis_buffer,
        }
    } // new


//    pub fn draw<T, U>(&self, target: &mut T, uniforms: &U)
//        where T: glium::Surface,
//              U: glium::uniforms::Uniforms
//    {
//        let axis_indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
//        target.draw(&self.axis_buffer,
//                  &axis_indices,
//                  &self.axis_program,
//                  uniforms,
//                  &Default::default())
//            .unwrap();
//    }
}

use renderer::context::{DrawItem, Program};

impl DrawItem for Axis{

    fn get_vertices<'a> (&'a self)-> &'a glium::vertex::VertexBufferAny{
        &self.axis_buffer
    }
    fn get_primitive(&self) -> glium::index::PrimitiveType{
        glium::index::PrimitiveType::LinesList
    }
}

impl Program for Axis{

    fn get_program<'a>(&'a self) -> &'a glium::Program{
        &self.axis_program
    }
}


#[cfg(test)]
mod utils {

    use super::index_vertex_list;

    // unidimensional test
    #[derive(Copy, Clone)]
    struct Vertex{
        point: (f32, f32, f32),
    }
    impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
            self.point.0 == other.point.0 &&
            self.point.1 == other.point.1 &&
            self.point.2 == other.point.2
        }
    }

    #[test]
    fn index_vertices()
    {   
        let mut v = Vec::new();
 
        v.push(Vertex { point: (0.0, 0.0, 0.0), });
        v.push(Vertex { point: (1.0, 0.0, 0.0), });
        v.push(Vertex { point: (0.0, 2.0, 0.0), });
        v.push(Vertex { point: (0.0, 0.0, 3.0), });

        v.push(Vertex { point: (0.0, 0.0, 0.0), });
        v.push(Vertex { point: (1.0, 0.0, 0.0), });
        v.push(Vertex { point: (0.0, 2.0, 0.0), });
        v.push(Vertex { point: (0.0, 0.0, 3.0), });
 
        let (vertices, indices) = index_vertex_list(&v);
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 8);
    }

}
