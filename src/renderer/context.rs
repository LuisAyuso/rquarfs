extern crate glium;

use renderer::texquad;

pub type Backend = glium::backend::glutin_backend::GlutinFacade;
pub type VerticesT = glium::vertex::VertexBufferAny;
pub type IndicesT =  glium::index::IndexBufferAny;
pub type PrimitiveT = glium::index::PrimitiveType;

#[derive(Copy, Clone)]
pub enum RenderType{
    Textured,
    WireFrame,
}

/// this class wraps up all render stuff,
/// glium should not be visible ouside of this... except for buffers?
/// the idea is to simplify te calls to draw, and wrap all intialization 
pub struct Context
{
    display_ptr: Backend,
}


impl Context
{
    pub fn new(width : u32, height : u32) -> Context
    {
        use glium::DisplayBuild;
        //use glium::debug::DebugCallbackBehavior;

        Context {
            display_ptr : glium::glutin::WindowBuilder::new()
                    .with_title("Quarfs!")
                    .with_dimensions(width, height)
                    .with_depth_buffer(24)
                    .with_srgb(Some(false))
                    //.build_glium_debug(DebugCallbackBehavior::PrintAll)
                    .build_glium()
                    .unwrap()
        }
    }

    pub fn display(&self) -> &Backend 
    {
        &self.display_ptr
    }

    pub fn resize(&mut self, w: u32, h: u32){
       print!("resize {}x{}\n", w, h);
//      TODO: change here the perspective matrix (which should be owned by ctx)
    }

} // context

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// draw temporary object
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct DrawSurface<'a>{
 //   ctx   : &'a Context,
    target : glium::Frame,
    render_params: glium::DrawParameters<'a>
}

impl<'a> DrawSurface<'a>{


    #[inline]
    pub fn gl_begin(ctx : &'a Context, render_type: RenderType) -> DrawSurface<'a>{
        use glium::Surface;
        let mut target = ctx.display().draw();
        target.clear_color_and_depth((0.2, 0.5, 0.4, 1.0), 1.0);
        DrawSurface {
     //       ctx: ctx, 
            target: target,
            render_params : glium::DrawParameters {
                backface_culling: glium::BackfaceCullingMode::CullClockwise,
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                polygon_mode: match render_type{
                    RenderType::WireFrame =>  glium::PolygonMode::Line,
                    _ =>   glium::PolygonMode::Fill,
                },
                ..Default::default()
            },
        }
    }

    #[inline]
    pub fn draw<O,U>(mut self, obj : &O, uniforms: &U)
        -> DrawSurface<'a>
    where O : DrawItem + Program, U: glium::uniforms::Uniforms
    {
        //println!("a");
        use glium::Surface;
        self.target.draw(obj.get_vertices(),
                         glium::index::NoIndices(obj.get_primitive()),
                         &obj.get_program(),
                         uniforms, 
                         &self.render_params).unwrap();
        self
    }

    #[inline]
    pub fn draw_instanciated_with_indices_and_program<O,P,U>(mut self, 
                                                             obj : &O, 
                                                             instances: &VerticesT, 
                                                             prg : &P, 
                                                             uniforms: &U) -> DrawSurface<'a>
    where O: DrawIndexed, P: Program, U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;
        self.target.draw((obj.get_vertices(), instances.per_instance().unwrap()),
                         obj.get_indices(),
                         prg.get_program(),
                         uniforms, 
                         &self.render_params).unwrap();
        self
    } 

    pub fn draw_tex_quad<T>(mut self, quad: &texquad::TexQuad, texture: T) -> DrawSurface<'a> 
        where T: glium::uniforms::AsUniformValue
    {

        //println!("c");
        use glium::Surface;

        // generate uniforms because i doint know how to return the uniforms type
        let quad_uniforms = uniform! {
            quad_texture: texture,
        };

		self.target.draw(quad.get_vertices(),
                         glium::index::NoIndices(quad.get_primitive()),
                         quad.get_program(),
                         &quad_uniforms,
                         &glium::DrawParameters {
                             //backface_culling: glium::BackfaceCullingMode::CullClockwise,
                             viewport: Some(glium::Rect{ 
                                 left: 10, 
                                 bottom: 10, 
                                 width: 400,
                                 height: 300,
                             }),
                             ..Default::default()
                         },).unwrap();
        //println!(" == ");
        self
    }

    #[inline]
    pub fn gl_end(self){
        self.target.finish().unwrap();
    }



} // impl ctx



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Traits:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait Program{
    fn get_program<'a>(&'a self) -> &'a glium::program::Program;
}

impl Program for glium::program::Program{
    fn get_program<'a>(&'a self) -> &'a glium::program::Program{
        &self
    }
}

/// is drawable if we can plot it right away.
/// we have a geometry and a program that should understand it
pub trait DrawItem {
    fn get_vertices<'a> (&'a self)-> &'a VerticesT;
    fn get_primitive(&self) -> PrimitiveT;
}

pub trait DrawIndexed {
    fn get_vertices<'a> (&'a self)-> &'a VerticesT;
    fn get_indices<'a> (&'a self) -> &'a IndicesT;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Tests:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  
#[cfg(test)]
mod tests {
    use world;
    use renderer::quadtree;
    use cgmath::{Point3, Vector3, Vector4, Matrix4, deg, perspective};
    use std::fmt::Debug;

    fn print<T> (v: &Vec<T>)
        where T : Debug
    {
        println!("vector contains:");
        v.iter().map(|elem|{
                println!("\t{:?}", elem);
            }).last();
    }

    #[test]
    fn pvm_checks() {
        use image::Pixel;
  
        print!("load height map \n");
        // read height map 
        let height = world::textures::load_rgb("assets/height.jpg");
        let height_dimensions = height.dimensions();

        // translations for the instances
        let size_x :f32 = height_dimensions.0 as f32;
        let size_z :f32 = height_dimensions.1 as f32;

        print!("gen matrices \n");

		let view = Matrix4::look_at(Point3::new(0.0, 75.0, -110.0),Point3::new(0.0, 0.0, -0.0), Vector3::new(0.0, 1.0, 0.0));
        let perspective: Matrix4<f32> = perspective(deg(45.0), 1.0, 2.0, 100.0);
        let model = Matrix4::from_translation(Vector3::new(-(size_x / 2.0), 0.0, -(size_z / 2.0)));
        let pvm = perspective * view * model;

        print!("test \n");

        let tree = quadtree::Patch::new((0,0), (size_x as u32 -1, size_z as u32 -1));
        let res = quadtree::test(100, tree, &|(x, z)|{
            let pixel = height.get_pixel(x,z);
            let components = pixel.channels();
            println!("({},{},{})", x, components[0], z);
            let v = Vector4::new(x as f32, components[0] as f32, z as f32, 1.0);

            let pos = pvm * v;
            println!("({:?} => {:?})", v, pos);

            let res = pos.x >= 0.0 && pos.x <= 1.0 && pos.y >= 0.0 && pos.y <= 1.0;
            println!("{}", res);

            res
        });
        print(&res);
        
    }
}
