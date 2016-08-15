extern crate glium;


type Backend = glium::backend::glutin_backend::GlutinFacade;
pub type VerticesT = glium::vertex::VertexBufferAny;
pub type IndicesT =  glium::IndexBuffer<u16>;
pub type PrimitiveT = glium::index::PrimitiveType;


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

        Context {
            display_ptr : glium::glutin::WindowBuilder::new()
                    .with_title("Quarfs!")
                    .with_dimensions(width, height)
                    .with_depth_buffer(24)
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
//      TODO: not implemented panic :(
//       use glium::DisplayBuild ;
//       let new = glium::glutin::WindowBuilder::new()
//                    .with_dimensions(w, h)
//                    .with_depth_buffer(24)
//                    .rebuild_glium(&*self.display_ptr)
//                    .unwrap();
    }

} // context


pub struct DrawSurface<'a>{
    ctx   : &'a Context,
    target : glium::Frame,
    render_params: glium::DrawParameters<'a>
}

impl<'a> DrawSurface<'a>{

    #[inline]
    pub fn frame_begin(ctx : &'a Context) -> DrawSurface<'a>{
        use glium::Surface;
        DrawSurface {
            ctx: ctx, 
            target: ctx.display().draw() ,
            render_params : glium::DrawParameters {
                backface_culling: glium::BackfaceCullingMode::CullClockwise,
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                // polygon_mode: glium::PolygonMode::Line,
                ..Default::default()
            },
        }
    }

    #[inline]
    pub fn draw_with_indices<O,U>(mut self, obj : &O, uniforms: &U) -> DrawSurface<'a>
    where O : DrawIndexed + Program, U: glium::uniforms::Uniforms
    {
        use glium::Surface;
        let vert = obj.get_vertices();
        self.target.draw(&vert,
                         obj.get_indices(), 
                         obj.get_program(),
                         uniforms, 
                         &self.render_params).unwrap();
        self
    }

    #[inline]
    pub fn draw<O,U>(mut self, obj : &O, uniforms: &U, )
        -> DrawSurface<'a>
    where O : DrawItem + Program, U: glium::uniforms::Uniforms
    {
        use glium::Surface;
        let &'a vert = &obj.get_vertices(); 
        self.target.draw(vert,
                         glium::index::NoIndices(obj.get_primitive()),
                         obj.get_program(),
                         uniforms, 
                         &self.render_params).unwrap();
        self
    }


    #[inline]
    pub fn draw_with_program<O,P,U>(mut self, obj : &O, prg : &P) -> DrawSurface<'a>
    where O: DrawItem, P: Program
    {
        //print!("draw {} {} with REAL program  \n",
        //    obj.get_vertices(), obj.get_indices());
        self
    }

    #[inline]
    pub fn frame_end(mut self){
        self.target.finish().unwrap();
    }
} // impl ctx


pub trait Program{
    fn get_program(&self) -> &glium::program::Program;
}

impl Program for glium::program::Program{
    fn get_program(&self) -> &glium::program::Program{
        &self
    }
}

/// a geometry has vertices and indicesgg
pub trait Geometry{
}

/// is drawable if we can plot it right away.
/// we have a geometry and a program that should understan it
pub trait DrawItem {
    fn get_vertices(self)-> VerticesT;
    fn get_primitive(&self) -> PrimitiveT;
}

pub trait DrawIndexed {
    fn get_vertices(self)-> VerticesT;
    fn get_indices(&self) -> & IndicesT;
}



