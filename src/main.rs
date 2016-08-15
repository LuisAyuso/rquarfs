
#[macro_use]
extern crate glium;
extern crate rand;
extern crate cgmath;
extern crate image;

mod model;
mod utils;

use std::io;
use std::io::Read;
use cgmath::{Point3, Vector3, Matrix4, Euler, deg, Quaternion, perspective};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
struct Vertex 
{
    position:  (f32, f32, f32),
    normal:    (f32, f32, f32),
    tex_coord: (f32, f32),
}

implement_vertex!(Vertex, position, normal, tex_coord);


#[derive(Copy, Clone)]
struct AxisVert 
{
    position: (f32, f32, f32),
    color:    (f32, f32, f32),
}

implement_vertex!(AxisVert, position, color);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
		Vertex { position: ( 0.0, 0.0, 0.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (0.0,0.0) }, // 0
		Vertex { position: ( 0.0, 1.0, 0.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (0.0,1.0) }, // 1
		Vertex { position: ( 1.0, 0.0, 0.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (1.0,0.0) }, // 2
		Vertex { position: ( 1.0, 1.0, 0.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (1.0,1.0) }, // 3

		Vertex { position: ( 0.0, 0.0, 1.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (1.0,1.0) }, // 4
		Vertex { position: ( 0.0, 1.0, 1.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (1.0,0.0) }, // 5
		Vertex { position: ( 1.0, 0.0, 1.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (0.0,1.0) }, // 6
		Vertex { position: ( 1.0, 1.0, 1.0 ), normal: ( 1.0, 1.0, 1.0), tex_coord: (0.0,0.0) }, // 7
    ]).unwrap();

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
											&[
                                                2, 0, 1, 2, 1, 3u16,
                                                6, 2, 3, 6, 3, 7u16,

                                                5, 1, 0, 5, 0, 4u16,
                                                3, 1, 5, 3, 5, 7u16,
                                                
                                                0, 2, 6, 0, 6, 4u16,
                                                7, 5, 4, 7, 4, 6u16,
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


    let axis_buffer = glium::VertexBuffer::new(&display, &[
		AxisVert { position: ( 0.0, 0.0, 0.0 ), color: ( 1.0, 0.0, 0.0),  },
		AxisVert { position: ( 2.0, 0.0, 0.0 ), color: ( 1.0, 0.0, 0.0),  },
		AxisVert { position: ( 0.0, 0.0, 0.0 ), color: ( 0.0, 1.0, 0.0),  },
		AxisVert { position: ( 0.0, 2.0, 0.0 ), color: ( 0.0, 1.0, 0.0),  },
		AxisVert { position: ( 0.0, 0.0, 0.0 ), color: ( 0.0, 0.0, 1.0),  },
		AxisVert { position: ( 0.0, 0.0, 2.0 ), color: ( 0.0, 0.0, 1.0),  },
    ]).unwrap();

    let axis_program = 
            glium::Program::from_source(&display, 
	 // vertex shader
        "
            #version 140

			in vec3 position;
			in vec3 color;

            uniform mat4 perspective_matrix;
            uniform mat4 view_matrix;

            out vec3 f_color;

            void main() {
	            gl_Position = perspective_matrix * view_matrix * vec4(position, 1.0);
                f_color = color;
            }
        ",

        // fragment shader
        "
            #version 140

            in vec3 f_color;
            out vec4 color;

            void main() {
				color = vec4(f_color, 1.0);
            }
		"
            , None).unwrap();

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    

    let atlas = model::textures::load_atlas("tex_pack").unwrap();
    let atlas_count = atlas.count;
    let atlas_side  = atlas.side;
    let image_dimensions = atlas.image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed( atlas.image.into_raw(), image_dimensions);
    //let atlas_texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let atlas_texture = glium::texture::Texture2d::with_mipmaps(&display, image,
                 glium::texture::MipmapsOption::AutoGeneratedMipmaps).unwrap();
	//let ref atlas_texture = model::textures::load_textures(&display, "tex_pack")[0];

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // translations for the instances
    let mut translations : Vec<(f32,f32)> = Vec::new();
	for x in 0..4 {
        for y in 0..4 {
            translations.push((x as f32,y as f32));
        }
	}

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // building the vertex buffer with the attributes per instance
    let instance_attr = {

        #[derive(Copy, Clone, Debug)]
        struct Attr {
            world_position: (f32, f32, f32),
            in_color:       (f32, f32, f32),
            tex_offset:     (f32, f32)
        }
        implement_vertex!(Attr, world_position, in_color, tex_offset);

        use rand::{Rng};
        let mut rng = rand::thread_rng();

        let mut count = 0;
        let data = translations.iter().map( |pos| {

            let tex_id = count % atlas_count; //rng.gen_range(0, atlas_count);
            count += 1;
            let i_off = ((tex_id % atlas_side) as f32) / atlas_side as f32;
            let j_off = ((tex_id / atlas_side) as f32) / atlas_side as f32;

            print!("{}  {},{} @ {},{}\n", tex_id, (tex_id % atlas_side), (tex_id / atlas_side), i_off, j_off);

            Attr {
                world_position: (pos.0, pos.1, count as f32 * 0.2),
                in_color:       (rand::random(), rand::random(), rand::random()),
                tex_offset:     (i_off as f32, j_off as f32), 
            }
        }).collect::<Vec<_>>();

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
    let view_eye: Point3<f32>    = Point3::new(0.0, 0.0,-5.0);
    let view_center: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    let view_up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

 	let perspective_matrix: Matrix4<f32> = perspective(deg(45.0), window_ratio, 0.0001, 1000.0);
    let mut view_matrix:    Matrix4<f32> = Matrix4::look_at(view_eye, view_center, view_up);
    let mut model_matrix:   Matrix4<f32> = Matrix4::from_translation(Vector3::new(-2.0,-2.0,0.0));

    // per increment rotation 
    let rotation = Matrix4::from(Quaternion::from(Euler {
        x: deg(0.0),
        y: deg(0.0),
        z: deg(0.0),
    }));

	// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    print!("{} instances\n", translations.len());

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~ RENDER LOOP ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

	utils::loop_with_report ( |_| {

        view_matrix = view_matrix * Matrix4::from_translation(Vector3::new(0.0,0.0, 0.0));
        model_matrix = model_matrix * rotation;

		let uniforms = uniform! {
			perspective_matrix: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
			view_matrix:        Into::<[[f32; 4]; 4]>::into(view_matrix),
			model_matrix:       Into::<[[f32; 4]; 4]>::into(model_matrix),

            atlas_texture: &atlas_texture,
            atlas_side:    atlas_side as u32,
        };

		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//    draw cubes
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        target.draw( 
				(&vertex_buffer, instance_attr.per_instance().unwrap()), 
				&indices, 
				&program, 
				&uniforms,
                &params,
			).unwrap();


		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//    draw axis
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


		let axis_indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
		target.draw(&axis_buffer, 
					&axis_indices, 
					&axis_program, 
					//&glium::uniforms::EmptyUniforms,
                    &uniforms,
                    &glium::draw_parameters::DrawParameters {
                        point_size: Some(3f32),
                        .. Default::default()
                    },
					//&Default::default()
                    ).unwrap();


		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
		//    finish frame
		// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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
	}, 0);

}
