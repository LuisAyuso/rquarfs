use glium;

pub type Backend = glium::backend::glutin_backend::GlutinFacade;
pub type VerticesT = glium::vertex::VertexBufferAny;
pub type IndicesT = glium::index::IndexBufferAny;
pub type PrimitiveT = glium::index::PrimitiveType;

#[derive(Copy, Clone)]
pub enum RenderType {
    Textured,
    WireFrame,
}

/// this class wraps up all render stuff,
/// glium should not be visible ouside of this... except for buffers?
/// the idea is to simplify te calls to draw, and wrap all intialization
pub struct Context {
    display_ptr: Backend,
    pub width: u32,
    pub height: u32,
}


impl Context {
    pub fn new(width: u32, height: u32) -> Context {
        use glium::DisplayBuild;
        //use glium::debug::DebugCallbackBehavior;

        Context {
            display_ptr: glium::glutin::WindowBuilder::new()
                        .with_title("Quarfs!")
                        .with_dimensions(width, height)
                        .with_depth_buffer(24)
                        .with_srgb(Some(false))
                        //.build_glium_debug(DebugCallbackBehavior::PrintAll)
                        .build_glium()
                    .unwrap(),
            width: width,
            height: height,
        }
    }

    #[allow(dead_code)]
    pub fn new_debug(width: u32, height: u32) -> Context {
        use glium::DisplayBuild;
        use glium::debug::DebugCallbackBehavior;

        Context {
            display_ptr: glium::glutin::WindowBuilder::new()
                .with_title("Quarfs!")
                .with_dimensions(width, height)
                .with_depth_buffer(24)
                .with_srgb(Some(false))
                .build_glium_debug(DebugCallbackBehavior::PrintAll)
                .unwrap(),
            width: width,
            height: height,
        }
    }

    pub fn display(&self) -> &Backend {
        &self.display_ptr
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        println!("resize {}x{}", w, h);
        self.width = w;
        self.height = h;
        //      TODO: change here the perspective matrix (which should be owned by ctx)
    }
} // context

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// draw temporary object
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct DrawSurface<'a> {
    //   ctx   : &'a Context,
    target: glium::Frame,
    render_params: glium::DrawParameters<'a>,
}

impl<'a> DrawSurface<'a> {
    #[inline]
    pub fn gl_begin(ctx: &'a Context, render_type: RenderType) -> DrawSurface<'a> {
        use glium::Surface;
        let mut target = ctx.display().draw();
        target.clear_color_and_depth((0.2, 0.5, 0.4, 1.0), 1.0);
        DrawSurface {
            //       ctx: ctx,
            target: target,
            render_params: glium::DrawParameters {
                backface_culling: glium::BackfaceCullingMode::CullClockwise,
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                polygon_mode: match render_type {
                    RenderType::WireFrame => glium::PolygonMode::Line,
                    _ => glium::PolygonMode::Fill,
                },
                ..Default::default()
            },
        }
    }

    #[inline]
    pub fn draw<O, U>(&mut self, obj: &O, uniforms: &U)
        where O: DrawItem + Program,
              U: glium::uniforms::Uniforms
    {
        //println!("a");
        use glium::Surface;
        self.target
            .draw(obj.get_vertices(),
                  glium::index::NoIndices(obj.get_primitive()),
                  obj.get_program(),
                  uniforms,
                  &self.render_params)
            .unwrap();
    }

    #[inline]
    pub fn draw_instanciated_with_indices_and_program<O, P, U>(&mut self,
                                                               obj: &O,
                                                               instances: &VerticesT,
                                                               prg: &P,
                                                               uniforms: &U)
        where O: DrawIndexed,
              P: Program,
              U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;
        self.target
            .draw((obj.get_vertices(), instances.per_instance().unwrap()),
                  obj.get_indices(),
                  prg.get_program(),
                  uniforms,
                  &self.render_params)
            .unwrap();
    }

    #[inline]
    pub fn draw_with_indices_and_program<O, P, U>(&mut self,
                                                  obj: &O,
                                                  prg: &P,
                                                  uniforms: &U)
        where O: DrawIndexed,
              P: Program,
              U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;
        let x = self.target.draw(obj.get_vertices(), 
                  obj.get_indices(),
                  prg.get_program(),
                  uniforms,
                  &self.render_params);

        if let Err(err) = x{
            println!("render error: {:?}",err);
        }
    }

    pub fn draw_overlay_quad<O, T>(&mut self, quad: &O, texture: T)  
        where O: DrawItem + Program,
              T: glium::uniforms::AsUniformValue
    {

        //println!("c");
        use glium::Surface;

        // generate uniforms because i doint know how to return the uniforms type
        let quad_uniforms = uniform! {
            quad_texture: texture,
        };

        self.target
            .draw(quad.get_vertices(),
                  glium::index::NoIndices(quad.get_primitive()),
                  quad.get_program(),
                  &quad_uniforms,
                  &glium::DrawParameters {
                      //backface_culling: glium::BackfaceCullingMode::CullClockwise,
                      viewport: Some(glium::Rect {
                          left: 10,
                          bottom: 10,
                          width: 300,
                          height: 300,
                      }),
                      ..Default::default()
                  })
            .unwrap();
        //println!(" == ");
    }

    #[inline]
    pub fn gl_end(self) {
        self.target.finish().unwrap();
    }
} // impl ctx



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Traits:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait Program{
    fn get_program(&self) -> &glium::program::Program;
    fn with_tess(&self) -> bool;
}

impl Program for glium::program::Program {
    fn get_program(&self) -> &glium::program::Program {
        self
    }
    fn with_tess(&self) -> bool{
        self.has_tessellation_shaders()
    }
}

/// is drawable if we can plot it right away.
/// we have a geometry and a program that should understand it
pub trait DrawItem {
    fn get_vertices(&self) -> &VerticesT;
    fn get_primitive(&self) -> PrimitiveT;
}

pub trait DrawIndexed {
    fn get_vertices(&self) -> &VerticesT;
    fn get_indices(&self) -> &IndicesT;
}
