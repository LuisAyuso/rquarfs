
use glium;

use renderer::context::*;
use glium::program::Program as GlProgram;
use std::boxed::Box as Box;


#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32, f32),
}
implement_vertex!(Vertex, position);

// ~~~~~~~~~~


// the idea here is to create a tessellation terrain,
// lets start with a grid.
pub struct Terrain{
    vertices: VerticesT
}


impl Terrain{

    pub fn new<F: glium::backend::Facade>(display: &F) -> Terrain {

        let vertices_buff = glium::VertexBuffer::new(display, &[
               Vertex { position: (0.0, 0.0, 0.0)}, 
               Vertex { position: (1.0, 0.0, 0.0)}, 
               Vertex { position: (1.0, 1.0, 0.0)}, 
               Vertex { position: (0.0, 1.0, 0.0)}]);


        Terrain{
            vertices: vertices_buff.unwrap().into()
        }
    }

}


impl DrawItem for Terrain {
    fn get_vertices(&self) -> &VerticesT{
        &self.vertices
    }
    fn get_primitive(&self) -> PrimitiveT{
        glium::index::PrimitiveType::Patches { vertices_per_patch: 4 }
    }
}
