
#[macro_use]
extern crate glium;
extern crate time;
extern crate glm;
extern crate rand;

use std::io;
use std::io::Read;
//use std::io::prelude::*;
use std::fs::{self, File};
use time::{PreciseTime, Duration};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

type Mat4 = [[f32; 4]; 4];

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex 
{
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn load_shader( path : &str) -> Result<String, io::Error> 
{

		let mut vs_path = fs::canonicalize(".").unwrap();
		vs_path.push("shaders");
		vs_path.push(format! ("{}{}", path, ".glsl"));
        print!("open: {:?}\n", vs_path);

		let mut f = try!(File::open(vs_path));

		let mut shader_buff = String::new();
		let _ = f.read_to_string(&mut shader_buff);
        return Ok(shader_buff);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn loop_with_report<F : FnMut()>(mut body : F )  
{
    loop 
	{
        let mut fps_accum :f64 = 0.0;
        let mut samples :u32 = 0;

        let start = PreciseTime::now();
        while start.to(PreciseTime::now()) < Duration::seconds(2) 
		{
            let start_t = time::precise_time_s();

            body();

            let end_t = time::precise_time_s();
            fps_accum += end_t-start_t;
            samples += 1;
        }

        print!("fps: {}\n", (samples as f64)/fps_accum );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn look_at( camera : glm::Vec3, center: glm::Vec3, up: glm::Vec3) -> Mat4 
{
	let mut matrix : Mat4 = [[0.0, 0.0, 0.0, 0.0],
							 [0.0, 0.0, 0.0, 0.0],
							 [0.0, 0.0, 0.0, 0.0],
							 [0.0, 0.0, 0.0, 0.0]];
//Create a new coordinate system:

	let z = camera - center;
//    Z.Normalize();
	let x = up * z;
	let y = z * x;

//Cross-product gives area of parallelogram, which is < 1.0 for non-perpendicular unit-length vectors; 
// so normalize X, Y here:

//    X.Normalize();
//    Y.Normalize();

//Put everything into the resulting 4x4 matrix:

    matrix[0][0] = x.x;
    matrix[1][0] = x.y;
    matrix[2][0] = x.z;
    //matrix[3][0] = -X.Dot( Eye );
    matrix[0][1] = y.x;
    matrix[1][1] = y.y;
    matrix[2][1] = y.z;
    //matrix[3][1] = -Y.Dot( Eye );
    matrix[0][2] = z.x;
    matrix[1][2] = z.y;
    matrix[2][2] = z.z;
    //matrix[3][2] = -Z.Dot( Eye );
    matrix[0][3] = 0.0;
    matrix[1][3] = 0.0;
    matrix[2][3] = 0.0;
    matrix[3][3] = 1.0;

print!("{:?}\n", matrix);

	matrix
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {

	let path = fs::canonicalize(".").unwrap();
	print! ("hello, we are in: {:?}\n", path);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1920, 1080)
        .build_glium().unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	let vertex1 = Vertex { position: [-0.5,  0.5] };
	let vertex2 = Vertex { position: [ 0.5, -0.5] };
	let vertex3 = Vertex { position: [-0.5, -0.5] };

	let vertex4 = Vertex { position: [-0.5,  0.5] };
	let vertex5 = Vertex { position: [ 0.5, -0.5] };
	let vertex6 = Vertex { position: [ 0.5,  0.5] };

	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    let vertex_shader = load_shader("geom.vs");
    let fragment_shader = load_shader("geom.fs");

//   print!("vertex: {:?}\n", vertex_shader);
//   print!("fragment: {:?}\n", fragment_shader);

    let program = 
        match (vertex_shader, fragment_shader) {
            (Ok(a), Ok(b)) => glium::Program::from_source(&display, &a, &b, None).unwrap(),
            _ => panic!("could not find shaders"),
        };


    // translations for the instances
    let mut translations : Vec<(f32,f32)> = Vec::new();
    let offset = 0.1;
	for x in 0..10 {
        for y in 0..10 {
            translations.push((x as f32 / 10.0 + offset, y as f32 / 10.0 + offset));
        }
	}

    // building the vertex buffer with the attributes per instance
    let instance_attr = {
        #[derive(Copy, Clone)]
        struct Attr {
            world_position: (f32, f32),
            in_color: (f32, f32, f32),
        }

        implement_vertex!(Attr, world_position, in_color);

        let data = translations.iter().map( |pos| {
            Attr {
                world_position: (pos.0, pos.1),
                in_color:       (rand::random(), rand::random(), rand::random()),
            }
        }).collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
    };

//	// parameters for rendering (culling)
//	let params = glium::DrawParameters {
//		depth: glium::Depth {
//			test: glium::DepthTest::IfLess,
//			write: true,
//			.. Default::default()
//		},
//		.. Default::default()
//	};

	// generate camera...
	let model_view = look_at(glm::vec3(0.0, 100.0, 0.0), 
							 glm::vec3(0.0, 0.0, 0.0),
							 glm::vec3(1.0, 0.0, 0.0));

	loop_with_report ( || {

		let uniforms = uniform! {
            model_view: model_view,
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw( 
				(&vertex_buffer, instance_attr.per_instance().unwrap()), 
				&indices, 
				&program, 
				&uniforms,
				//&glium::uniforms::EmptyUniforms, 
				&Default::default()
			).unwrap();
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => std::process::exit(0),   // the window has been closed by the user
                _ => ()
            }
        }
	});

}
