extern crate time;

pub mod context;
pub mod camera;
pub mod shader;
pub mod texquad;
pub mod shadowmapper;
mod ss_pass;
pub mod graphs;
pub mod pipeline;

mod geometry_manager;

pub type ScreenSpacePass<'a> = ss_pass::ScreenSpacePass<'a>;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    convert to vertex + index
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
pub fn index_vertex_list<T>(vertices_org: &[T]) -> (Vec<T>, Vec<u32>)
    where T: PartialEq + Clone
{

    let mut vertices: Vec<T> = Vec::new();
    let mut indices: Vec<u32> = Vec::with_capacity(vertices_org.len());

    // for each vertex, search in vertices list, if not there, insert the last one.
    for v in vertices_org {
        let pos = vertices.iter().position(|r| *r == *v);
        match pos {
            None => {
                indices.push(vertices.len() as u32);
                vertices.push(v.clone());
            }
            Some(x) => indices.push(x as u32),
        }
    }
    // vertices.shrink_to_fit();
    // indices.shrink_to_fit();

    (vertices, indices)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    convert to vertex + index
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tools {

    use super::index_vertex_list;

    // unidimensional test
    #[derive(Copy, Clone)]
    struct Vertex {
        point: (f32, f32, f32),
    }
    impl PartialEq for Vertex {
        fn eq(&self, other: &Vertex) -> bool {
            self.point.0 == other.point.0 && self.point.1 == other.point.1 &&
            self.point.2 == other.point.2
        }
    }

    #[test]
    fn index_vertices() {
        let mut v = Vec::new();

        v.push(Vertex { point: (0.0, 0.0, 0.0) });
        v.push(Vertex { point: (1.0, 0.0, 0.0) });
        v.push(Vertex { point: (0.0, 2.0, 0.0) });
        v.push(Vertex { point: (0.0, 0.0, 3.0) });

        v.push(Vertex { point: (0.0, 0.0, 0.0) });
        v.push(Vertex { point: (1.0, 0.0, 0.0) });
        v.push(Vertex { point: (0.0, 2.0, 0.0) });
        v.push(Vertex { point: (0.0, 0.0, 3.0) });

        let (vertices, indices) = index_vertex_list(&v);
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 8);
    }

}
