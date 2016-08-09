
#[macro_use]
extern crate glium;

use std::io;
use std::io::prelude::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {

	use std::fs::{self, File};

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


//	let (vertex_shader_src, fragment_shader_src) = {

		let mut vs_path = fs::canonicalize(".").unwrap();
		vs_path.push("shaders");
		vs_path.push("geom.vs.glsl");

		let mut fs_path = fs::canonicalize(".").unwrap();
		fs_path.push("shaders");
		fs_path.push("geom.vs.glsl");

		print! ("vertex: {:?}\n", vs_path);
		print! ("fragment: {:?}\n", fs_path);

//		let mut f1 = try!(File::open(vs_path));
//		let mut vertex_shader_buff = String::new();
//		let len1 = f1.read_to_string(&mut vertex_shader_buff);
//
//		let mut f2 = try!(File::open(fs_path));
//		let mut fragment_shader_buff = String::new();
//		let len2 = f2.read_to_string(&mut fragment_shader_buff);
//
//		(vertex_shader_buff, fragment_shader_buff)
//	};


//	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
//
//    let mut t: f32 = -0.5;
//
//	loop {
//
//        t += 0.0002;
//        if t > 0.5 {
//            t = -0.5;
//        }
//
//        let mut target = display.draw();
//        target.clear_color(0.0, 0.0, 1.0, 1.0);
//		// target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
//		target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t }, &Default::default()).unwrap();
//        target.finish().unwrap();
//
//		// listing the events produced by the window and waiting to be received
//		for ev in display.poll_events() {
//			match ev {
//				glium::glutin::Event::Closed => return,   // the window has been closed by the user
//				_ => ()
//			}
//		}
//	}

}
