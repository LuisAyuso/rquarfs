
#[macro_use]
extern crate glium;

use std::io;
use std::io::Read;
//use std::io::prelude::*;
use std::fs::{self, File};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn load_shader( path : &str) -> Result<String, io::Error> {

		let mut vs_path = fs::canonicalize(".").unwrap();
		vs_path.push("shaders");
		vs_path.push(format! ("{}{}", path, ".glsl"));
        print!("open: {:?}\n", vs_path);

		let mut f = try!(File::open(vs_path));

		let mut shader_buff = String::new();
		f.read_to_string(&mut shader_buff);
        return Ok(shader_buff);
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {


	let path = fs::canonicalize(".").unwrap();
	print! ("hello, we are in: {:?}\n", path);

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [ 0.0,  0.5] };
	let vertex3 = Vertex { position: [ 0.5, -0.25] };
	let shape = vec![vertex1, vertex2, vertex3];

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


    let mut t: f32 = -0.5;

	loop {

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
		// target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
		target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t }, &Default::default()).unwrap();
        target.finish().unwrap();

		// listing the events produced by the window and waiting to be received
		for ev in display.poll_events() {
			match ev {
				glium::glutin::Event::Closed => return,   // the window has been closed by the user
				_ => ()
			}
		}
	}

}
