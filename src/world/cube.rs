extern crate glium;


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
    tex_coord: (f32, f32),
}
implement_vertex!(Vertex, position, normal, tex_coord);

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//use glium::vertex::VertexBufferAny;
//use glium::index::IndexBufferAny;

pub struct Cube{
    vertices: glium::vertex::VertexBufferAny,
    indices: glium::index::IndexBufferAny,
}

impl Cube{

    pub fn new<F: glium::backend::Facade>(display: &F) ->Option<Cube>{
        let vertices = glium::VertexBuffer::new(display,
             &[
               Vertex { position: (-0.5, -0.5, -0.5),normal: ( 0.0,-1.0, 0.0), tex_coord: (0.0, 0.0), }, //-0
               Vertex { position: (-0.5, 0.5, -0.5), normal: ( 0.0, 0.0,-1.0), tex_coord: (1.0, 0.0), }, //-1
               Vertex { position: (0.5, -0.5, -0.5), normal: ( 8.0, 0.0, 8.0), tex_coord: (0.0, 1.0), }, // 2
               Vertex { position: (0.5, 0.5, -0.5),  normal: (-1.0, 0.0, 0.0), tex_coord: (1.0, 1.0), }, //-3

               Vertex { position: (-0.5, -0.5, 0.5), normal: ( 1.0, 0.0, 0.0), tex_coord: (0.0, 1.0), }, //-4 
               Vertex { position: (-0.5, 0.5, 0.5),  normal: ( 0.0, 8.0, 0.0), tex_coord: (1.0, 1.0), }, // 5
               Vertex { position: (0.5, -0.5, 0.5),  normal: ( 8.0, 8.0, 0.0), tex_coord: (0.0, 0.0), }, // 6
               Vertex { position: (0.5, 0.5, 0.5),   normal: ( 0.0, 0.0, 1.0), tex_coord: (1.0, 0.0), }, //-7 

               Vertex { position: (-0.5, -0.5, 0.5), normal: ( 0.0, 0.0, 8.0), tex_coord: (1.0, 0.0), }, // 4' 8
               Vertex { position: (-0.5, 0.5, 0.5),  normal: ( 0.0, 1.0, 0.0), tex_coord: (0.0, 0.0), }, //-5' 9
               Vertex { position: (0.5, -0.5, 0.5),  normal: ( 0.0, 8.0, 0.0), tex_coord: (1.0, 1.0), }, // 6'10
               Vertex { position: (0.5, 0.5, 0.5),   normal: ( 0.0, 8.0, 8.0), tex_coord: (0.0, 1.0), }  // 7'11 
             ]);

        let indices = glium::IndexBuffer::new(display,
                                glium::index::PrimitiveType::TrianglesList,
                                &[5, 1, 4, 1, 0, 4u16, // normals in 4  
                                  2, 0, 1, 3, 2, 1u16, // normals in 1 
                                  6, 2, 3, 7, 6, 3u16, // normals in 3 
                                  4, 6, 7, 5, 4, 7u16, // normals in 7 
                                 10, 8, 0, 2,10, 0u16, // normals in 0 
                                  3, 1, 9,11, 3, 9u16, // normals in 9 
                                ]);

        Some(Cube  { 
            vertices : vertices.unwrap().into(),
            indices : indices.unwrap().into(),
        })
    } // new

} // impl Cube

use renderer::context::DrawIndexed;

impl DrawIndexed for Cube{

    fn get_vertices<'a> (&'a self)-> &'a glium::vertex::VertexBufferAny{
        &self.vertices
    }
    fn get_indices<'a> (&'a self) -> &'a glium::index::IndexBufferAny{
        &self.indices
    }
}

