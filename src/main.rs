#![feature(test)]

#[macro_use]
extern crate glium;
extern crate rand;
extern crate cgmath;
extern crate image;
extern crate glutin;
extern crate time;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod world;
mod utils;
mod renderer;

#[warn(unused_imports)]
use cgmath::{Point3, Vector3, Matrix4, Euler, deg, perspective, Transform};
use renderer::context;
use renderer::camera;
use renderer::shader;
use renderer::texquad;
use world::image_atlas as img_atlas;
// use renderer::pipeline::*;
// use world::cube;
// use renderer::shadowmapper;

use renderer::context::DrawIndexed;
use renderer::context::Program;
use glium::Surface;


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

enum Preview {
    Prepass,
    Height,
    Depth,
    Color,
    SSAO,
    Blur,
    Noise,
}

fn main() {

    let window_ratio: f32 = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let mut ctx = context::Context::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    println!("load height map ");
    // read height map
    let height = img_atlas::load_rgb("assets/D18.png");
    let height_dimensions = height.dimensions();

    // translations for the instances
    let size_x: f32 = height_dimensions.0 as f32;
    let size_z: f32 = height_dimensions.1 as f32;

    //  map overlay ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let mut quad = texquad::TexQuad::new(&ctx);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let height_raw = glium::texture::RawImage2d::from_raw_rgb(height.into_raw(), height_dimensions);
    let height_map = glium::texture::Texture2d::new(ctx.display(), height_raw).unwrap();

    let color = img_atlas::load_rgb("assets/C18W.png");
    let dim = color.dimensions();
    let color_raw = glium::texture::RawImage2d::from_raw_rgb(color.into_raw(), dim);
    let color_map = glium::texture::Texture2d::new(ctx.display(), color_raw).unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // generate camera...
    let eye = Point3::new(10.0, 90.0, 250.0);
    let looking = Point3::new(0.0, 0.0, 0.0); // Point3::new(0.0, 0.0, -10.0);
    let mut cam = camera::Camera::new(eye, looking);

    const NEAR: f32 = 5.0;
    const FAR: f32 = 1500.0;

    let mut perspective_matrix: Matrix4<f32> = perspective(deg(45.0), window_ratio, NEAR, FAR);
    let mut model_matrix: Matrix4<f32> =
        Matrix4::from_translation(Vector3::new(-(size_x as f32 / 2.0),
                                               0.0,
                                               -(size_z as f32 / 2.0)));

    // per increment rotation
    let rotation = Quaternion::from(Euler {
        x: deg(0.0),
        y: deg(0.05),
        z: deg(0.0),
    });

    let rot_mat = Matrix4::from(rotation);
    let _ = rot_mat;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use renderer::context::DrawSurface;
    use renderer::context::RenderType;
    use cgmath::Rotation;
    use cgmath::Quaternion;
    let mut run = true;
    let mut compute_shadows = false;
    let mut render_kind = RenderType::Textured;

    // sun pos
    let mut sun_pos = Point3::new(0.0, 75.0, size_x as f32); // / 2.0 + 20.0);
    let sun_rot = Quaternion::from(Euler {
        x: deg(0.02),
        y: deg(0.02),
        z: deg(0.0),
    });

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let new_terrain = world::terrain::Terrain::new(&ctx, size_x as u32, size_z as u32);

    let terrain_prg = shader::ProgramReloader::new(&ctx, "terrain_texture");
    if terrain_prg.is_err() {
        std::process::exit(-1);
    }
    let mut terrain_prg = terrain_prg.unwrap();

    let terrain_normals_prg = shader::ProgramReloader::new(&ctx, "terrain_normals");
    if terrain_normals_prg.is_err() {
        std::process::exit(-1);
    }
    let mut terrain_normals_prg = terrain_normals_prg.unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let axis_plot = utils::Axis::new(&ctx);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    use glium::texture;
    let (h, w) = ctx.get_size();
    let prepass_texture =
        texture::Texture2d::empty_with_format(ctx.display(),
                                              texture::UncompressedFloatFormat::F32F32F32F32,
                                              texture::MipmapsOption::NoMipmap,
                                              h,
                                              w)
            .unwrap();

    let depth_tex = texture::DepthTexture2d::empty_with_format(ctx.display(),
                                                               texture::DepthFormat::F32,
                                                               texture::MipmapsOption::NoMipmap,
                                                               h,
                                                               w)
        .unwrap();



    let mut prepas_frame =
        Box::new(glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(),
                                                                          &prepass_texture,
                                                                          &depth_tex)
            .unwrap());

    //  ssao pass  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let noise_img = img_atlas::generate_noise((w, h));
    let noise_dim = noise_img.dimensions();
    let noise_tex = glium::texture::RawImage2d::from_raw_rgb(noise_img.into_raw(), noise_dim);
    let noise_tex = glium::texture::Texture2d::new(ctx.display(), noise_tex).unwrap();

    let drop_depth = texture::DepthTexture2d::empty_with_format(ctx.display(),
                                                                texture::DepthFormat::F32,
                                                                texture::MipmapsOption::NoMipmap,
                                                                h,
                                                                w)
        .unwrap();

    let ssao_texture =
        texture::Texture2d::empty_with_format(ctx.display(),
                                              texture::UncompressedFloatFormat::F32F32F32F32,
                                              texture::MipmapsOption::NoMipmap,
                                              h,
                                              w)
            .unwrap();
    let mut ssao = renderer::ScreenSpacePass::new(&ctx, "ssao", &ssao_texture, &drop_depth);

    //  blur  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let blur_texture =
        texture::Texture2d::empty_with_format(ctx.display(),
                                              texture::UncompressedFloatFormat::F32F32F32F32,
                                              texture::MipmapsOption::NoMipmap,
                                              h,
                                              w)
            .unwrap();
    let mut blur = renderer::ScreenSpacePass::new(&ctx, "blur", &blur_texture, &drop_depth);

    // performance

    let mut performance_program = shader::ProgramReloader::new(&ctx, "performance").unwrap();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~ RENDER LOOP ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let mut preview = Preview::Blur;
    let mut chunk_size: u32 = 20;
    utils::loop_with_report(&mut |delta: f64, _: &mut utils::PerformaceCounters| {

        cam.update(delta as f32);
        terrain_prg.update(&ctx, delta);
        terrain_normals_prg.update(&ctx, delta);
        quad.update(&ctx, delta);
        ssao.update(&ctx, delta);
        blur.update(&ctx, delta);
        performance_program.update(&ctx, delta);

        // keep mut separated
        {
            let cam_mat: Matrix4<f32> = cam.into();
            let view_matrix = cam_mat * Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

            if run {
                model_matrix = rot_mat * model_matrix;
                //  model_matrix = model_matrix;
                sun_pos = sun_rot.rotate_point(sun_pos);
            }

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //   matrix
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            let pvm = perspective_matrix * view_matrix * model_matrix;
            let inverse_matrix = pvm.inverse_transform().unwrap();

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //    render scene
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            let uniforms = uniform! {
                perspective: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
                view:        Into::<[[f32; 4]; 4]>::into(view_matrix),
                model:       Into::<[[f32; 4]; 4]>::into(model_matrix),
                pvm:         Into::<[[f32; 4]; 4]>::into(pvm),
                //light_space_matrix: Into::<[[f32; 4]; 4]>::into(light_space_matrix),

                //atlas_texture: &atlas_texture,
                //atlas_side:    atlas_side as u32,
                //shadow_texture: shadow_maker.depth_as_texture(),

                sun_pos:    Into::<[f32; 3]>::into(sun_pos),
                cam_pos:    Into::<[f32; 3]>::into(cam.get_eye()),
                shadows_enabled:   compute_shadows,

                height_map: &height_map,
                height_size:    (size_x as u32, size_z as u32),

                screen_size: ctx.get_size(),
                color_map: &color_map,

                ssao_texture: &blur_texture,
            };


            // ~~~~~~~~~ prepass: normals and depth  ~~~~~~~~~~~~~~~~

            let parameters = glium::DrawParameters {
                backface_culling: glium::BackfaceCullingMode::CullClockwise,
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                polygon_mode: glium::PolygonMode::Fill,
                provoking_vertex: glium::draw_parameters::ProvokingVertex::LastVertex,
                ..Default::default()
            };

            prepas_frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
            prepas_frame.draw((new_terrain.get_vertices(),
                       new_terrain.get_tiles()
                          .per_instance()
                          .unwrap()),
                      new_terrain.get_indices(),
                      terrain_normals_prg.get_program(),
                      &uniforms,
                      &parameters)
                .unwrap();

            // ~~~~~~~~~  SSAO ~~~~~~~~~~~~~~~~

            ssao.execute_pass(&inverse_matrix, &prepass_texture, &depth_tex, &noise_tex);

            // ~~~~~~~~~  blur SSAO ~~~~~~~~~~~~~~~~

            blur.execute_pass(&inverse_matrix, &ssao_texture, &depth_tex, &noise_tex);

            // ~~~~~~~~~  render color ~~~~~~~~~~~~~~~~

            let mut surface = DrawSurface::gl_begin(&ctx, render_kind);
            surface.draw(&axis_plot, &uniforms);
            // surface.draw_with_indices_and_program(&new_terrain, &terrain_prg, &uniforms);
            surface.draw_instanciated_with_indices_and_program(&new_terrain,
                                                               new_terrain.get_tiles(),
                                                               &terrain_prg,
                                                               &uniforms);

            match preview {
                Preview::Noise => surface.draw_overlay_quad(&quad, &noise_tex, false),
                Preview::SSAO => surface.draw_overlay_quad(&quad, &ssao_texture, false),
                Preview::Blur => surface.draw_overlay_quad(&quad, &blur_texture, false),
                Preview::Prepass => surface.draw_overlay_quad(&quad, &prepass_texture, false),
                Preview::Height => surface.draw_overlay_quad(&quad, &height_map, false),
                Preview::Depth => surface.draw_overlay_quad(&quad, &depth_tex, true),
                Preview::Color => surface.draw_overlay_quad(&quad, &color_map, false),
            };

            surface.gl_end();
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
                    Event::Closed |
                    Event::KeyboardInput(_, 9, _) => std::process::exit(0),  // esc
                    Event::KeyboardInput(ElementState::Released, 33, _) => {
                        run = !run;
                    }
                    Event::KeyboardInput(ElementState::Released, 0, _) => {
                        compute_shadows = !compute_shadows;
                        println!("toggle shadows");
                    }
                    Event::KeyboardInput(ElementState::Released, 24, _) => {
                        render_kind = RenderType::Textured
                    }
                    Event::KeyboardInput(ElementState::Released, 25, _) => {
                        render_kind = RenderType::WireFrame
                    }
                    Event::KeyboardInput(ElementState::Released, 30, _) => {
                        cam.move_to(Point3::new(0.0, 65.0, -110.0))
                    }
                    Event::KeyboardInput(ElementState::Released, 86, _) => chunk_size += 10,
                    Event::KeyboardInput(ElementState::Released, 82, _) => chunk_size -= 10,

                    Event::KeyboardInput(ElementState::Released, 14, _) => {
                        println!("preview Noise");
                        preview = Preview::Noise;
                    }
                    Event::KeyboardInput(ElementState::Released, 15, _) => {
                        println!("preview Blur");
                        preview = Preview::Blur;
                    }
                    Event::KeyboardInput(ElementState::Released, 16, _) => {
                        println!("preview SSAO");
                        preview = Preview::SSAO;
                    }
                    Event::KeyboardInput(ElementState::Released, 17, _) => {
                        println!("preview Prepass");
                        preview = Preview::Prepass;
                    }
                    Event::KeyboardInput(ElementState::Released, 18, _) => {
                        println!("preview Height");
                        preview = Preview::Height;
                    }
                    Event::KeyboardInput(ElementState::Released, 19, _) => {
                        println!("preview Color");
                        preview = Preview::Color;
                    }
                    Event::KeyboardInput(ElementState::Released, 20, _) => {
                        println!("preview Depth");
                        preview = Preview::Depth;
                    }
                    Event::KeyboardInput(_, x, _) => println!("key {}", x),
                    Event::Resized(w, h) => resizes.push((w, h)),
                    Event::MouseWheel(x, _) => {
                        if let glium::glutin::MouseScrollDelta::LineDelta(_, y) = x {
                            cam.change_elevation(y * 5.0);
                        }
                    }
                    _ => (),
                }
            }
        }

        // can not change window while context is borrowed
        for (w, h) in resizes {
            ctx.resize(w, h);
            // FIXME, this is a fix
            perspective_matrix = perspective(deg(45.0), w as f32 / h as f32, NEAR, FAR);


            ctx.get_size();
            // texture = texture::Texture2d::empty_with_format(ctx.display(),
            //                                                texture::UncompressedFloatFormat::F32,
            //                                                texture::MipmapsOption::NoMipmap,
            //                                                h, w).unwrap();

            // depth = texture::DepthTexture2d::empty_with_format(ctx.display(),
            //                                                   texture::DepthFormat::F32,
            //                                                   texture::MipmapsOption::NoMipmap,
            //                                                    h, w).unwrap();
        }

    },
                            1); // refresh every 5 secs

}
