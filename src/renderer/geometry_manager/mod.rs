
use glium::vertex::*;
use glium::index::*;

use super::context::Context;
use super::context::IdType;
use std::marker::PhantomData;

use std::collections::BTreeMap;
use std::ops::Deref;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub type VerticesT = VertexBufferAny;
pub type IndicesBufT = IndexBufferAny;
pub type PrimitiveT = PrimitiveType;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

trait Geomerty {
    fn get_vertices(&self) -> &VerticesT;
    fn get_indices(&self) -> &IndicesT;
    fn get_primitive(&self) -> &PrimitiveT;
}

enum IndicesT {
    NoIdx(PrimitiveT),
    Idx(IndicesBufT),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct GeomertyInstance<T> {
    name: String,
    vertices: VerticesT,
    indices: IndicesT,
    primitive: PrimitiveT,
    vertices_type: PhantomData<T>,
}

impl<T> Geomerty for GeomertyInstance<T> {

    fn get_vertices(&self) -> &VerticesT{
        &self.vertices
    }
    fn get_indices(&self) -> &IndicesT{
        &self.indices
    }
    fn get_primitive(&self) -> &PrimitiveT{
        &self.primitive
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// TODO: move to context
#[derive(Copy, Clone, Debug)]
enum ManagerError {
    ItemRedefinition,
}

struct GeomertyManager {
    cache: BTreeMap<IdType, Box<Geomerty>>,
}


impl GeomertyManager {
    fn new() -> GeomertyManager {
        GeomertyManager { cache: BTreeMap::new() }
    }

    // create a geomerty from data
    fn create_geom_from_data<T>(&mut self,
                                ctx: &mut Context,
                                name: &str,
                                data: &[T],
                                kind: PrimitiveT)
                                -> Result<IdType, ManagerError>
        where T: Vertex + Send + 'static
    {
        let id = ctx.get_id_for(name);
        if self.cache.contains_key(&id) {
            return Err(ManagerError::ItemRedefinition);
        }

        let vertices = VertexBuffer::new(ctx.display(), data).unwrap();
        let g: Box<GeomertyInstance<T>> = Box::new(GeomertyInstance {
            name: name.to_string(),
            vertices: vertices.into(),
            indices: IndicesT::NoIdx(kind),
            primitive: kind,
            vertices_type: PhantomData,
        });

        self.cache.insert(id, g);
        Ok(id)
    }

    fn create_geom_from_data_with_indices<T>(&mut self,
                                             ctx: &mut Context,
                                             name: &str,
                                             data: &[T],
                                             indices: &[u32],
                                             kind: PrimitiveT)
                                             -> Result<IdType, ManagerError>
        where T: Vertex + Send + 'static
    {
        let id = ctx.get_id_for(name);
        if self.cache.contains_key(&id) {
            return Err(ManagerError::ItemRedefinition);
        }

        let vertices = VertexBuffer::new(ctx.display(), data).unwrap();
        let indices = IndexBuffer::new(ctx.display(), kind, indices).unwrap();
        let g: Box<GeomertyInstance<T>> = Box::new(GeomertyInstance {
            name: name.to_string(),
            vertices: vertices.into(),
            indices: IndicesT::Idx(indices.into()),
            primitive: kind,
            vertices_type: PhantomData,
        });

        self.cache.insert(id, g);
        Ok(id)
    }

    fn get_geom(&self, id: IdType) -> Option<&Geomerty>{
        if let Some(x) = self.cache.get(&id){
            return Some(x.deref());
        }
        None
    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let _ = GeomertyManager::new();
    }

    #[test]
    fn create_geom_no_indices() {

        #[derive(Copy, Clone)]
        struct MyVertices {
            vert: (f32, f32, f32),
        }

        implement_vertex!(MyVertices, vert);

        let mut ctx = Context::new_empty();
        let mut mgr = GeomertyManager::new();

        let a = mgr.create_geom_from_data(&mut ctx,
                                          "test",
                                          &[MyVertices { vert: (1.0, 1.0, 1.0) }],
                                          PrimitiveType::TrianglesList);
        assert!(a.is_ok());
        {
            let v = mgr.get_geom(a.unwrap());
            let v = v.expect("should exist");
            assert_eq!(v.get_vertices().len(), 1);
        }

        let b = mgr.create_geom_from_data(&mut ctx,
                                          "test",
                                          &[MyVertices { vert: (1.0, 1.0, 1.0) }],
                                          PrimitiveType::TrianglesList);
        assert!(b.is_err());
    }

    #[test]
    fn create_geom_indices() {

        #[derive(Copy, Clone)]
        struct MyVertices {
            vert: (f32, f32),
        }

        implement_vertex!(MyVertices, vert);

        let mut ctx = Context::new_empty();
        let mut mgr = GeomertyManager::new();

        let a = mgr.create_geom_from_data_with_indices(&mut ctx,
                                                       "test",
                                                       &[MyVertices { vert: (1.0, 1.0) },
                                                         MyVertices { vert: (1.0, 1.0) }],
                                                       &[0, 1, 2],
                                                       PrimitiveType::TrianglesList);
        assert!(a.is_ok());
        {
            let v = mgr.get_geom(a.unwrap());
            let v = v.expect("should exist");
            assert_eq!(v.get_vertices().len(), 2);
        }

        let b = mgr.create_geom_from_data_with_indices(&mut ctx,
                                                       "test",
                                                       &[MyVertices { vert: (1.0, 1.0) }],
                                                       &[0, 1, 2],
                                                       PrimitiveType::TrianglesList);
        assert!(b.is_err());
    }
}
