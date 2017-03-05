use glium;
use rand;
use renderer::context::*;

// ~~~~~~~~~~


/// The idea here is to create a tessellation terrain,
pub struct Terrain{
    vertices: VerticesT,
    tiles: VerticesT,
    indices: IndicesT,
}


impl Terrain{

    /// crate a terrain object of a certain dimmensions.
    /// it will be tiled in 64x64 sized tiles (which is the maximun tessellation we can get with
    /// resolution 1 to 1)
    pub fn new<F: glium::backend::Facade>(display: &F, width: u32, height: u32) -> Terrain {
        use rand::Rng;

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        #[derive(Copy, Clone)]
        struct Vertex {
            position: (u32, u32),
        }
        implement_vertex!(Vertex, position);
 
        let vertices_buff = glium::VertexBuffer::new(display, &[
               Vertex { position: (  0,  0)}, 
               Vertex { position: ( 64,  0)}, 
               Vertex { position: (  0, 64)}, 
               Vertex { position: ( 64, 64)}]);

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        #[derive(Copy, Clone, Debug)]
        struct Tile {
            tile_offset: (u32, u32, u32),
        }
        implement_vertex!(Tile, tile_offset);

        let mut rng = rand::thread_rng();

        let mut data: Vec<Tile> = Vec::new();
        for i in 0..(width/64)-1{
            for j in 0..(width/64)-1{
                let detail = rng.gen_range(0, 7);
                data.push(Tile{ tile_offset: (i,j, detail)});
            }
        }

        let tiles = glium::vertex::VertexBuffer::dynamic(display, &data);

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let indices = glium::IndexBuffer::new(display,
                                              glium::index::PrimitiveType::Patches{ vertices_per_patch: 4},
                                              &[0u16, 2, 3, 1]);

        Terrain{
            vertices: vertices_buff.unwrap().into(),
            tiles: tiles.unwrap().into(),
            indices: indices.unwrap().into(),
        }
    }

    pub fn get_tiles(&self) -> &VerticesT{
        &self.tiles
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
