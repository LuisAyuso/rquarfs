

// the idea: some structure which, given an object, can instanciate all the 
// triangles needed to store it, and can keep a continuous allocated buffer
// in gpu which holds all triangles from all registered objects
//
// Index, can it perform automatic indexing? this should not be that difficult
// when update, it can mark the objects which are not registered and compact the gpu buffer.
//
//
//  buffer A:  vertices (compacted, vertices might be shared)
//  buffer B: indices (contiguous for each object)
//  map:      hash Object -> chunk in indices (offset, lenght)
//
//  question, what happens with: texture coordinates? normals? 

use std::collections::BTreeMap;
use std::vec::Vec;
use std::cmp::PartialEq;
use std::hash::{Hash, SipHasher, Hasher};


type Chunk = (usize, usize);

/// contains a map, hash -> chunk in buffer, which tells us where is stored the data for a given
/// object (index)
pub struct ChunkManager<T> {
    vertices: Vec<T>,
    indices: Vec<usize>,
    map: BTreeMap<u64, Chunk>,
}

impl<T> ChunkManager<T>
    where T: PartialEq 
{

    pub fn new() -> ChunkManager<T>{
        ChunkManager{
            vertices: Vec::new(),
            indices: Vec::new(),
            map: BTreeMap::new(),
        }
    }

    pub fn add_chunk<O>(&mut self, object: &O)
    where O: VertexGenerator<T> + Hash {
        let hash = hash(object);
        if self.map.contains_key(&hash) { return; }

        let vertices = object.get_vertices();
        let mut obj_indx = Vec::<usize>::with_capacity(vertices.len());

        // for each vertex, search in vertices list, if not there, insert the last one.
        for v in vertices{
            let pos = self.vertices.iter().position(|r| *r == v);
            match pos{
                None => { self.vertices.push(v); obj_indx.push((self.vertices.len() - 1) as usize); },
                Some(x) => obj_indx.push(x as usize),
            }
        }
        
        let chunk = (self.indices.len(), obj_indx.len());
        self.indices.append(&mut obj_indx);

        self.map.insert(hash, chunk);
    }

    pub fn vertices (&self) -> &Vec<T>{
        &self.vertices
    }
    pub fn indices (&self) -> &Vec<usize>{
        &self.indices
    }
}


/// this trait needs to be implemented by the objects, so they can generate vertices (maybe
/// something else as well, like normals, tex coords)
pub trait VertexGenerator<T>{

    fn get_vertices(&self) -> Vec<T>;
}


fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod chunks {

    use super::ChunkManager;
    use super::VertexGenerator;

    // unidimensional test
    #[derive(Hash)]
    struct Segment{
        begin: u32,
        lenght: u32,
    }

    impl VertexGenerator<u32> for Segment
    {
        fn get_vertices(&self) -> Vec<u32>{
            let mut x = Vec::with_capacity(2);
            x.push(self.begin);
            x.push(self.begin+self.lenght);
            x
        }
    }


    #[test]
    fn ctor()
    {   
       ChunkManager::<u32>::new();
    }

    #[test]
    fn add()
    {   
       let mut mgr = ChunkManager::<u32>::new();

       assert_eq!(mgr.vertices.len(), 0);
       assert_eq!(mgr.indices.len(), 0);

       let a = Segment { begin: 0, lenght: 10 };
       let b = Segment { begin: 10, lenght: 10 };
       let c = Segment { begin: 0, lenght: 20 };


       mgr.add_chunk(&a);
       assert_eq!(mgr.vertices.len(), 2);
       assert_eq!(mgr.indices.len(), 2);

       mgr.add_chunk(&b);
       assert_eq!(mgr.vertices.len(), 3);
       assert_eq!(mgr.indices.len(), 4);

       mgr.add_chunk(&c);
       assert_eq!(mgr.vertices.len(), 3);
       assert_eq!(mgr.indices.len(), 6);
    }


}
