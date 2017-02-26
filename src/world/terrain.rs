use glium;
use renderer::context::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32, f32),
}
implement_vertex!(Vertex, position);

// ~~~~~~~~~~


// the idea here is to create a tessellation terrain,
// lets start with a grid.
pub struct Terrain{
    vertices: VerticesT,
    indices: IndicesT
}


impl Terrain{

    pub fn new<F: glium::backend::Facade>(display: &F, width: f32, height: f32) -> Terrain {
 
        let vertices_buff = glium::VertexBuffer::new(display, &[
               Vertex { position: (          0.0, 0.0, 0.0)}, 
               Vertex { position: ( width as f32, 0.0, 0.0)}, 
               Vertex { position: (          0.0, 0.0, height as f32)}, 
               Vertex { position: ( width as f32, 0.0, height as f32)}]);


        let indices = glium::IndexBuffer::new(display,
                                              glium::index::PrimitiveType::Patches{ vertices_per_patch: 4},
                                              &[0u16, 2, 3, 1]);

        Terrain{
            vertices: vertices_buff.unwrap().into(),
            indices: indices.unwrap().into(),
        }
    }

}


impl DrawIndexed for Terrain {
    fn get_vertices(&self) -> &VerticesT{
        &self.vertices
    }
    fn get_indices(&self) -> &IndicesT{
        &self.indices
    }
}
