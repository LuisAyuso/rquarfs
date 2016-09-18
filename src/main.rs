
#[macro_use]
extern crate glium;
extern crate rand;
extern crate cgmath;
extern crate image;
extern crate glutin;
extern crate time;


mod world;
mod utils;
mod renderer;
 
 #[warn(unused_imports)] 
use cgmath::{Point3, Vector3, Matrix4, Euler, deg, perspective};
use world::cube;
use renderer::context;
use renderer::camera;
use renderer::shader;
use renderer::texquad;
use renderer::shadowmapper;
//use rand::Rng;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {

    let window_ratio: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let mut ctx = context::Context::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let prg_tmp = shader::ProgramReloader::new(ctx.display(), "geom.vs", "geom.fs");
    if prg_tmp.is_err(){
         std::process::exit(-1);
    }
    let mut program = prg_tmp.unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let cube = cube::Cube::new(ctx.display()).unwrap();


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    print!("load atlas:\n");
    let atlas = world::textures::load_atlas("tex_pack").unwrap();
    //let atlas = world::textures::load_atlas("test/atlas2").unwrap();
    let atlas_count = atlas.count;
    let atlas_side = atlas.side;
    let image_dimensions = atlas.image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(atlas.image.into_raw(), image_dimensions);
    // let atlas_texture = glium::texture::Texture2d::new(ctx.display(), image).unwrap();
    let atlas_texture = glium::texture::Texture2d::with_mipmaps(ctx.display(), image,
                 glium::texture::MipmapsOption::AutoGeneratedMipmaps).unwrap();
    print!("loaded {} mipmaps:\n", atlas_texture.get_mipmap_levels());

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    print!("load height map \n");
    // read height map 
    let height = world::textures::load_rgb("assets/height.jpg");
    //let height = world::textures::load_rgb("assets/height_small.png");
    //let height = world::textures::load_rgb("assets/pico.png");
    //let height = world::textures::load_rgb("assets/moon.png");
    //let height = world::textures::load_rgb("assets/test.png");
    let height_dimensions = height.dimensions();

    // translations for the instances
    let size_x :f32 = height_dimensions.0 as f32;
    let size_z :f32 = height_dimensions.1 as f32;

    // round to closest power of 2
   // let size_x = size_x.log(2.0).ceil().powi(2);
   // let size_z = size_z.log(2.0).ceil().powi(2);

    let mut translations: Vec<(f32, f32, f32)> = Vec::new();
    for x in 0..size_x as u32{
        for y in 0..size_z as u32{
            // get height in coordinates x,y
            let h = world::textures::get_coords_height(&height, x, y);
            translations.push((x as f32, y as f32, h));
        }
    }
    //  Shadow mapping ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let shadow_maker = shadowmapper::ShadowMapper::new(&ctx);


    //  map overlay ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let quad = texquad::TexQuad::new(&ctx);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // building the vertex buffer with the attributes per instance
    let instance_attr : context::VerticesT = {

        #[derive(Copy, Clone, Debug)]
        struct Attr {
            world_position: (f32, f32, f32),
            in_color: (f32, f32, f32),
            tex_offset: (f32, f32),
            vox_height: f32,
        }
        implement_vertex!(Attr, world_position, in_color, tex_offset, vox_height);

        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut count = 0;
        let data = translations.iter()
            .map(|pos| {

                let tex_id = rng.gen_range(0, atlas_count);
                count += 1;
                let i_off = ((tex_id / atlas_side) as f32) / atlas_side as f32;
                let j_off = ((tex_id % atlas_side) as f32) / atlas_side as f32;

                //   print!("{}  {},{} @ {},{}\n", tex_id,
                //                                 (tex_id % atlas_side), (tex_id / atlas_side),
                //                                 i_off, j_off);
             
                let h = world::textures::get_max_neighbour(&height, pos.0 as u32, pos.1 as u32);

                Attr {
                    world_position: (pos.0, pos.2, pos.1),
                    in_color: (rand::random(), rand::random(), rand::random()),
                    tex_offset: (i_off as f32, j_off as f32),
                    vox_height: h,
                }
            })
            .collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(ctx.display(), &data).unwrap().into()
    };

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    let mut los = los::Los::new(&height);
    let lospreview = los::LosPreview::new(&ctx);

    let height_raw = glium::texture::RawImage2d::from_raw_rgb(height.into_raw(), height_dimensions);
    let height_map = glium::texture::Texture2d::new(ctx.display(), height_raw).unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // generate camera...
    let eye = Point3::new(10.0, 50.0, -150.0); 
    let looking = Point3::new(0.0, 0.0, 0.0); // Point3::new(0.0, 0.0, -10.0);
	let mut cam =  camera::Camera::new(eye, looking);
    
    const NEAR : f32 = 5.0;
    const FAR : f32 = 1500.0;

    let mut perspective_matrix: Matrix4<f32> = perspective(deg(45.0), window_ratio, NEAR, FAR);
    let mut model_matrix: Matrix4<f32> =
        Matrix4::from_translation(Vector3::new(-(size_x as f32 / 2.0), 0.0, -(size_z as f32 / 2.0)));
        //Matrix4::from_translation(Vector3::new(100.0, 0.0, 0.0));

    // per increment rotation
    //let rotation = Matrix4::from(Quaternion::from(Euler {
    let rotation = Quaternion::from(Euler {
        x: deg(0.0),
        y: deg(0.1),
        z: deg(0.0),
    });

    let rot_mat = Matrix4::from(rotation);
    let _ = rot_mat;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    print!("{} instances\n", translations.len());

    use renderer::context::DrawSurface;
    use renderer::context::RenderType;
    use renderer::los;
    use cgmath::Rotation;
    use cgmath::Quaternion;
    let mut run = true;
    let mut render_kind = RenderType::Textured;

    // sun pos
    let mut sun_pos = Point3::new(0.0, 150.0, 250.0);
    let sun_rot = Quaternion::from(Euler {
        x: deg(0.1),
        y: deg(0.1),
        z: deg(0.0),
    });


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let axis_plot = utils::Axis::new(ctx.display());


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~ RENDER LOOP ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let mut chunk_size: u32 = 20;
    utils::loop_with_report(&mut|delta:f64| {

        cam.update(delta as f32);
        program.update(ctx.display(), delta);

        // keep mut separated
        {
            let cam_mat : Matrix4<f32> = cam.into();
            let view_matrix = cam_mat * Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

            if run {
             //   model_matrix = rot_mat * model_matrix;
              //  model_matrix = model_matrix;
                sun_pos =  sun_rot.rotate_point(sun_pos);
            }

            // print!("{:?}\n", sun_pos);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //    cast shadows
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            //new view matrix, from the sun
		    let sun_view_mat = Matrix4::look_at(sun_pos, Point3::new(0.0,0.0,0.0), Vector3::new(0.0,1.0,0.0));
            let sun_perspective = cgmath::ortho(-512.0, 512.0,-512.0, 512.0, NEAR, FAR);
            let light_space_matrix = sun_perspective * sun_view_mat;

            // new uniforms
            let uniforms = uniform! {
                light_space_matrix: Into::<[[f32; 4]; 4]>::into(light_space_matrix),
                model:              Into::<[[f32; 4]; 4]>::into(model_matrix),
            };

            shadow_maker.compute_depth(&ctx, &cube, &instance_attr, &uniforms);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //    line of sight  
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            let pvm = perspective_matrix * view_matrix * model_matrix;
            los.update_view(chunk_size, &pvm);

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //    render scene 
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            let uniforms = uniform! {
                perspective: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
                view:        Into::<[[f32; 4]; 4]>::into(view_matrix),
                model:       Into::<[[f32; 4]; 4]>::into(model_matrix),
                light_space_matrix: Into::<[[f32; 4]; 4]>::into(light_space_matrix),

                atlas_texture: &atlas_texture,
                shadow_texture: shadow_maker.depth_as_texture(),
                atlas_side:    atlas_side as u32,
                sun_pos:    Into::<[f32; 3]>::into(sun_pos),
            };

            let losquad = lospreview.get_drawable(&ctx, &los);
            
            DrawSurface::gl_begin(&ctx, render_kind)
                            .draw(&axis_plot, &uniforms)
                            .draw_instanciated_with_indices_and_program(&cube, 
                                                                        &instance_attr, 
                                                                        &program, 
                                                                        &uniforms)
                            //.draw_overlay_quad(&quad, shadow_maker.depth_as_texture())
                            .draw_overlay_quad(&quad, &height_map)
                            .draw_overlay_quad(&losquad, &height_map)
                        .gl_end();

          }


        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //    event handling
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // listing the events produced by the window and waiting to be received
        let mut resizes = Vec::new();
        {
            let events = ctx.display().poll_events();
            for ev in events {

                use glium::glutin::Event;
                use glium::glutin::ElementState;
                match ev {
                    Event::Closed => std::process::exit(0),  // the window has been closed 
                    Event::KeyboardInput(_, 9, _) => std::process::exit(0),  // esc
                    Event::KeyboardInput(ElementState::Released, 33, _) => run = !run,
                    Event::KeyboardInput(ElementState::Released, 32, _)
                        => render_kind = RenderType::Textured,
                    Event::KeyboardInput(ElementState::Released, 31, _) 
                        => render_kind = RenderType::WireFrame,
                    Event::KeyboardInput(ElementState::Released, 30, _)  =>
                                    cam.move_to(Point3::new(0.0, 65.0,-110.0)),
                    Event::KeyboardInput(ElementState::Released, 86, _)  => chunk_size += 10,
                    Event::KeyboardInput(ElementState::Released, 82, _)  => chunk_size -= 10,
                    Event::KeyboardInput(_, x, _) => print!("key {}\n", x),
                    Event::Resized(w, h) => resizes.push((w,h)),
                    Event::MouseWheel(x,_) => match x{
                            glutin::MouseScrollDelta::LineDelta(_, y) =>
                                    cam.change_elevation(y*5.0),
                            _ => (),
                    },
                    _ => (),
                }
            }
        }
        
        // can not change window while context is borrowed
        for (w, h) in resizes{
            ctx.resize(w,h);
            // FIXME, this is a fix
            perspective_matrix = perspective(deg(45.0), w as f32 / h as f32, NEAR, FAR);
        }

    }, 10); // refresh every 10 secs

}
