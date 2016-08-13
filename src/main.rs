
#[macro_use]
extern crate glium;
extern crate time;
extern crate glm;
extern crate rand;
extern crate cgmath;


//mod model;

use std::io;
use std::io::Read;
//use std::io::prelude::*;
use std::fs::{self, File};
use time::{PreciseTime, Duration};

use cgmath::{Point3, Vector3, Matrix4, SquareMatrix, Euler, deg, Quaternion, perspective};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex 
{
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn load_shader( path : &str) -> Result<String, io::Error> 
{

		let mut vs_path = fs::canonicalize(".").unwrap();
		vs_path.push("shaders");
		vs_path.push(format! ("{}{}", path, ".glsl"));
        print!("load: {:?}\n", vs_path);

		let mut f = try!(File::open(vs_path));

		let mut shader_buff = String::new();
		let _ = f.read_to_string(&mut shader_buff);
        return Ok(shader_buff);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn loop_with_report<F : FnMut(f64)>(mut body : F )  
{
    loop 
	{
        let mut fps_accum :f64 = 0.0;
        let mut samples :u32 = 0;
        let mut delta : f64 = 0.0;

        let start = PreciseTime::now();
        while start.to(PreciseTime::now()) < Duration::seconds(2) 
		{
            let start_t = time::precise_time_s();

            body(delta);

            let end_t = time::precise_time_s();
            delta = end_t-start_t;
            fps_accum = delta;
            samples += 1;
        }

        print!("fps: {}\n", (samples as f64)/fps_accum );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {

	let window_width = 1920;
	let window_height = 1080;
	let window_ratio : f32 = window_width as f32 / window_height as f32;

	let path = fs::canonicalize(".").unwrap();
	print! ("hello, we are in: {:?}\n", path);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(window_width, window_height)
        .build_glium().unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5,-0.5, 0.5 ), }, 
		Vertex { position: ( -0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5, 0.5,-0.5 ), }, 
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5, 0.5,-0.5 ), }, 
		Vertex { position: (  0.5,-0.5, 0.5 ), }, 
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: (  0.5,-0.5,-0.5 ), }, 
		Vertex { position: (  0.5, 0.5,-0.5 ), }, 
		Vertex { position: (  0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5, 0.5, 0.5 ), }, 
		Vertex { position: ( -0.5, 0.5,-0.5 ), }, 
		Vertex { position: (  0.5,-0.5, 0.5 ), }, 
		Vertex { position: ( -0.5,-0.5, 0.5 ), }, 
		Vertex { position: ( -0.5,-0.5,-0.5 ), }, 
		Vertex { position: ( -0.5, 0.5, 0.5 ), }, 
		Vertex { position: ( -0.5,-0.5, 0.5 ), }, 
		Vertex { position: (  0.5,-0.5, 0.5 ), }, 
		Vertex { position: (  0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5,-0.5,-0.5 ), }, 
		Vertex { position: (  0.5, 0.5,-0.5 ), }, 
		Vertex { position: (  0.5,-0.5,-0.5 ), }, 
		Vertex { position: (  0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5,-0.5, 0.5 ), }, 
		Vertex { position: (  0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5, 0.5,-0.5 ), }, 
		Vertex { position: ( -0.5, 0.5,-0.5 ), }, 
		Vertex { position: (  0.5, 0.5, 0.5 ), }, 
		Vertex { position: ( -0.5, 0.5,-0.5 ), }, 
		Vertex { position: ( -0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5, 0.5, 0.5 ), }, 
		Vertex { position: ( -0.5, 0.5, 0.5 ), }, 
		Vertex { position: (  0.5,-0.5, 0.5 ), },
    ]).unwrap();

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
    let view_eye: Point3<f32> = Point3::new(0.0, 0.0, 15.0);
    let view_center: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    let view_up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
 	let perspective_matrix: Matrix4<f32> = perspective(deg(45.0), window_ratio, 0.0001, 1000.0);
    let view_matrix: Matrix4<f32> = Matrix4::look_at(view_eye, view_center, view_up);
    let mut model_matrix: Matrix4<f32> = Matrix4::identity();

    // per increment rotation 
    let rotation = Matrix4::from(Quaternion::from(Euler {
        x: deg(1.0),
        y: deg(1.0),
        z: deg(0.0),
    }));

    let mut timestep = 0.0;
	loop_with_report ( |delta| {

        model_matrix = model_matrix * rotation;

		let uniforms = uniform! {
			perspective_matrix: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
			view_matrix: Into::<[[f32; 4]; 4]>::into(view_matrix),
			model_matrix: Into::<[[f32; 4]; 4]>::into(model_matrix),

        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw( 
				(&vertex_buffer, instance_attr.per_instance().unwrap()), 
				&indices, 
				&program, 
				&uniforms,
				//&glium::uniforms::EmptyUniforms, 
                //&params,
				&Default::default()
			).unwrap();
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => std::process::exit(0),   // the window has been closed 
                _ => ()
            }
        }
	});

}
