use glium;
use glutin;
use std::collections::BTreeMap;
//use glium::glutin::HeadlessRendererBuilder;

pub type Backend = glium::backend::glutin::GlutinBackend;
pub type Display = glium::backend::glutin::Display;
pub type EventsLoop = glutin::EventsLoop;
pub type VerticesT = glium::vertex::VertexBufferAny;
pub type IndicesT = glium::index::IndexBufferAny;
pub type PrimitiveT = glium::index::PrimitiveType;
pub type IdType = usize;

#[derive(Copy, Clone, Debug)]
pub enum ManagerError {
    ItemRedefinition,
    FailToCreateContext,
    BackEndErrror,
}

#[derive(Copy, Clone)]
pub enum RenderType {
    Textured,
    WireFrame,
}

#[derive(Copy, Clone, Debug)]
pub enum ContextError {
    HeadlessNotSupported,
    ContextNotSupported,
}


/// this class wraps up all render stuff,
/// glium should not be visible ouside of this... except for buffers?
/// the idea is to simplify te calls to draw, and wrap all intialization
pub struct Context {

    events_loop: EventsLoop,
    display: Display,
    id_cache: BTreeMap<String, IdType>,
    pub width: u32,
    pub height: u32,
}


impl Context {
    pub fn new(width: u32, height: u32) -> Result<Context, ContextError> {

        let events_loop = glutin::EventsLoop::new();

        let window_builder = glutin::WindowBuilder::new()
            .with_title("Quarfs!")
            .with_dimensions(width, height);

        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context, &events_loop).unwrap();

        Ok(Context {
            events_loop : events_loop,
            display: display,
            id_cache: BTreeMap::new(),
            width: width,
            height: height,
        })
    }

    #[cfg(test)]
    pub fn new_headless(w: u32, h: u32) -> Result<Context, ContextError> {
        use glium::DisplayBuild;

        let display = glium::glutin::HeadlessRendererBuilder::new(w, h).build_glium();

        if display.is_err() {
            return Err(ContextError::HeadlessNotSupported);
        }

        Ok(Context {
            display: display.unwrap(),
            id_cache: BTreeMap::new(),
            width: 0,
            height: 0,
        })
    }

    pub fn get_id_for(&mut self, name: &str) -> IdType {
        if let Some(x) = self.id_cache.get(&name.to_string()) {
            return *x;
        }
        let id = self.id_cache.len();
        self.id_cache.insert(name.to_string(), id);
        id
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn events_loop(&mut self) -> &mut EventsLoop{
        &mut self.events_loop
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        println!("resize {}x{}", w, h);
        self.width = w;
        self.height = h;
        //      TODO: change here the perspective matrix (which should be owned by ctx)
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
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
    pub fn get_frame(&mut self) -> &mut glium::Frame {
        &mut self.target
    }

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
        // println!("a");
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
        // println!("b");
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
    #[allow(dead_code)]
    pub fn draw_with_indices_and_program<O, P, U>(&mut self, obj: &O, prg: &P, uniforms: &U)
        where O: DrawIndexed,
              P: Program,
              U: glium::uniforms::Uniforms
    {
        // println!("b");
        use glium::Surface;
        let x = self.target.draw(obj.get_vertices(),
                                 obj.get_indices(),
                                 prg.get_program(),
                                 uniforms,
                                 &self.render_params);

        if let Err(err) = x {
            println!("render error: {:?}", err);
        }
    }

    pub fn draw_overlay_quad<O, T>(&mut self, quad: &O, texture: T, is_depth: bool)
        where O: DrawItem + Program,
              T: glium::uniforms::AsUniformValue
    {

        // println!("c");
        use glium::Surface;

        // generate uniforms because i doint know how to return the uniforms type
        let quad_uniforms = uniform! {
            quad_texture: texture,
            is_depth: is_depth,
        };

        self.target
            .draw(quad.get_vertices(),
                  glium::index::NoIndices(quad.get_primitive()),
                  quad.get_program(),
                  &quad_uniforms,
                  &glium::DrawParameters {
                      // backface_culling: glium::BackfaceCullingMode::CullClockwise,
                      viewport: Some(glium::Rect {
                          left: 10,
                          bottom: 10,
                          width: 640,
                          height: 480,
                      }),
                      ..Default::default()
                  })
            .unwrap();
        // println!(" == ");
    }

    #[inline]
    pub fn gl_end(self) {
        self.target.finish().unwrap();
    }
} // impl ctx


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Traits:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait Program {
    fn get_program(&self) -> &glium::program::Program;
    fn with_tess(&self) -> bool;
}

impl Program for glium::program::Program {
    fn get_program(&self) -> &glium::program::Program {
        self
    }
    fn with_tess(&self) -> bool {
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
