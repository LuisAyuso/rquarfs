
#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
extern crate cgmath;
extern crate image;


use std::io;
use std::io::Read;
use time::{PreciseTime, Duration};
use cgmath::{Point3, Vector3, Matrix4, Euler, deg, Quaternion, perspective};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex 
{
    position:  (f32, f32, f32),
    normal:    (f32, f32, f32),
    tex_coord: (f32, f32),
}

implement_vertex!(Vertex, position, normal, tex_coord);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn load_shader( name : &str) -> Result<String, io::Error> 
{
	use std::fs::{self, File};

	let mut path = fs::canonicalize(".").unwrap();
	path.push("shaders");
	path.push(format! ("{}{}", name, ".glsl"));
	print!("load shader: {:?}\n", path);

	let mut f = try!(File::open(path));

	let mut shader_buff = String::new();
	let _ = f.read_to_string(&mut shader_buff);
	return Ok(shader_buff);
}
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn load_textures<F: glium::backend::Facade> (display: &F,  set : &str) -> Vec<glium::texture::Texture2d>
{
	let mut path = fs::canonicalize(".").unwrap();
	path.push("assets");
	path.push(set);
	print!("load textures: {:?}\n", path);

    let mut textures = Vec::new();

    // iterate over textures:
	use std::fs;
	for entry in fs::read_dir(path).unwrap() {
		let dir = entry.unwrap();
		let image = image::open(dir.path()).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();

		print!(" {:?} -> {}x{}\n", dir.file_name(), image_dimensions.0, image_dimensions.1);
        textures.push(texture)
	}

    textures
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
        while start.to(PreciseTime::now()) < Duration::seconds(5) 
		{
            let start_t = time::precise_time_s();

            body(delta);

            let end_t = time::precise_time_s();
            delta = end_t-start_t;
            fps_accum += delta;
            samples += 1;
        }

        print!("fps: {} \n", (samples as f64)/fps_accum);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {

	let window_width = 1920;
	let window_height = 1080;
	let window_ratio : f32 = window_width as f32 / window_height as f32;

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(window_width, window_height)
        .with_depth_buffer(24)
        .build_glium().unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vertex { position: ( 0.0, 0.0, 1.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (0.0,0.0) }, 
		Vertex { position: ( 1.0, 0.0, 1.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (1.0,0.0) }, 
		Vertex { position: ( 1.0, 1.0, 1.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (1.0,1.0) }, 
		Vertex { position: ( 0.0, 1.0, 1.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (0.0,1.0) }, 

		Vertex { position: ( 0.0, 0.0, 0.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (0.0,0.0) }, 
		Vertex { position: ( 1.0, 0.0, 0.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (1.0,0.0) }, 
		Vertex { position: ( 1.0, 1.0, 0.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (1.0,1.0) }, 
		Vertex { position: ( 0.0, 1.0, 0.0 ), normal: ( 0.0, 0.0, 0.0), tex_coord: (0.0,1.0) }, 
    ]).unwrap();

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
											&[
											// front
											0, 1, 2,  2, 3, 0,
											// top
											1, 5, 6,  6, 2, 1,
											// back
											7, 6, 5,  5, 4, 7,
											// bottom
											4, 0, 3,  3, 7, 4,
											// left
											4, 5, 1,  1, 0, 4,
											// right
											3, 2, 6,  6, 7, 3u16
                                           ]).unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let vertex_shader = load_shader("geom.vs");
    let fragment_shader = load_shader("geom.fs");

    let program = 
        match (vertex_shader, fragment_shader) {
            (Ok(a), Ok(b)) => glium::Program::from_source(&display, &a, &b, None).unwrap(),
            _ => panic!("could not find shaders"),
        };

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	let textures = load_textures(&display, "tex_pack");

    // translations for the instances
    let mut translations : Vec<(f32,f32)> = Vec::new();
	for x in 0..10 {
        for y in 0..10 {
            translations.push((x as f32,y as f32));
        }
	}

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // building the vertex buffer with the attributes per instance
    let instance_attr = {

        #[derive(Copy, Clone, Debug)]
        struct Attr {
            world_position: (f32, f32),
            in_color: (f32, f32, f32),
            texture:  u32,
        }
        implement_vertex!(Attr, world_position, in_color, texture);

        use rand::{Rng};
        let mut rng = rand::thread_rng();

        let data = translations.iter().map( |pos| {
            Attr {
                world_position: (pos.0, pos.1),
                in_color:       (rand::random(), rand::random(), rand::random()),
                texture:        rng.gen_range(0, 6),
            }
        }).collect::<Vec<_>>();

        print!("{:?}\n", data);

        glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
    };

	// parameters for rendering (culling)
	let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
		depth: glium::Depth {
			test: glium::DepthTest::IfLess,
            write: true,
			.. Default::default()
		},
      //  polygon_mode: glium::PolygonMode::Line,
		.. Default::default()
	};

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	// generate camera...
    let view_eye: Point3<f32> = Point3::new(5.0, 5.0, 15.0);
    let view_center: Point3<f32> = Point3::new(5.0, 5.0, 0.0);

    let view_up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

 	let perspective_matrix: Matrix4<f32> = perspective(deg(45.0), window_ratio, 0.0001, 1000.0);
    let mut view_matrix:    Matrix4<f32> = Matrix4::look_at(view_eye, view_center, view_up);
    let mut model_matrix:   Matrix4<f32> = Matrix4::from_translation(Vector3::new(0.0,0.0,0.0));

    // per increment rotation 
    let rotation = Matrix4::from(Quaternion::from(Euler {
        x: deg(0.05),
        y: deg(0.05),
        z: deg(0.0),
    }));

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    print!("{} instances\n", translations.len());

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~ RENDER LOOP ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	loop_with_report ( |_| {

        view_matrix = view_matrix * Matrix4::from_translation(Vector3::new(0.0,0.0, 0.0));
        model_matrix = model_matrix * rotation;

		let uniforms = uniform! {
			perspective_matrix: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
			view_matrix:        Into::<[[f32; 4]; 4]>::into(view_matrix),
			model_matrix:       Into::<[[f32; 4]; 4]>::into(model_matrix),

            tex1 :  &textures[0],
            tex2 :  &textures[1],
            tex3 :  &textures[2],
            tex4 :  &textures[3],
            tex5 :  &textures[4],
            tex5 :  &textures[5],
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        target.draw( 
				(&vertex_buffer, instance_attr.per_instance().unwrap()), 
				&indices, 
				&program, 
				&uniforms,
                &params,
			).unwrap();
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {

            use glium::glutin::Event;

            match ev {
               Event::Closed => std::process::exit(0),   // the window has been closed 
               Event::KeyboardInput(_, 9, _) => std::process::exit(0),  
                _ => ()
            }
        }
	});

}
