// the idea: some structure which, given an object, can instanciate all the
// triangles needed to store it, and can keep a continuous allocated buffer
// in gpu which holds all triangles from all registered objects
//
// when update, it can mark the objects which are not registered and compact the gpu buffer.

use std::collections::BTreeMap;
use std::vec::Vec;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Clone)]
struct Chunk {
    offset: usize,
    size: usize,
    sequence: u64,
}

/// buffer A:  vertices (compacted, vertices might be shared)
/// buffer B: indices (contiguous for each object)
/// map:      hash Object -> chunk in indices (offset, lenght)
pub struct ChunkManager<Ver> {
    vertices: Vec<Ver>,
    indices: Vec<usize>,
    map: BTreeMap<u64, Chunk>,
    sequence: u64,
    free: Vec<Chunk>,
}

impl<Ver> ChunkManager<Ver>
    where Ver: PartialEq
{
    pub fn new() -> ChunkManager<Ver> {
        ChunkManager {
            vertices: Vec::new(),
            indices: Vec::new(),
            map: BTreeMap::new(),
            sequence: 0,
            free: Vec::with_capacity(0),
        }
    }

    /// inserts a chunk, reuses memory if possible (indices)
    fn insert_chunk(&mut self, mut indices: &mut Vec<usize>) -> Chunk {

        //if self.free.len() == 0 {

        let chunk = Chunk {
            offset: self.indices.len(),
            size: indices.len(),
            sequence: self.sequence,
        };

        self.indices.append(&mut indices);
        chunk
        //}
    }


    /// inserts one object in the manager, prevents duplicates
    pub fn add_object<O>(&mut self, object: &O)
        where O: VertexGenerator<Ver> + Hash
    {
        let hash = hash(object);
        if self.map.contains_key(&hash) {
            self.map.get_mut(&hash).unwrap().sequence = self.sequence;
            return;
        }

        let vertices = object.get_vertices();
        let mut obj_indx = Vec::<usize>::with_capacity(vertices.len());

        // for each vertex, search in vertices list, if not there, insert the last one.
        for v in vertices {
            let pos = self.vertices.iter().position(|r| *r == v);
            match pos {
                None => {
                    self.vertices.push(v);
                    obj_indx.push((self.vertices.len() - 1) as usize);
                }
                Some(x) => obj_indx.push(x as usize),
            }
        }

        let chunk = self.insert_chunk(&mut obj_indx);

        self.map.insert(hash, chunk);
    }

    /// insert a list of objects in the manager
    /// keeps track of unused chunks and marks to remove.
    pub fn add_batch<O>(&mut self, objects: &Vec<O>)
        where O: VertexGenerator<Ver> + Hash
    {
        self.sequence += 1;

        for obj in objects.iter() {
            self.add_object(obj);
        }

        let mut to_remove = Vec::new();
        for (key, chunk) in &self.map {
            if chunk.sequence != self.sequence {
                self.free.push(chunk.clone());
                to_remove.push(*key);
            }
        }

        for key in to_remove {
            self.map.remove(&key);
        }
    }

    pub fn vertices(&self) -> &Vec<Ver> {
        &self.vertices
    }
    pub fn indices(&self) -> &Vec<usize> {
        &self.indices
    }
}


/// this trait needs to be implemented by the objects, so they can generate vertices (maybe
/// something else as well, like normals, tex coords)
pub trait VertexGenerator<Ver> {
    fn get_vertices(&self) -> Vec<Ver>;
}


fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod chunks {

    use super::ChunkManager;
    use super::VertexGenerator;

    // unidimensional test
    #[derive(Hash)]
    struct Segment {
        begin: u32,
        lenght: u32,
    }

    impl VertexGenerator<u32> for Segment {
        fn get_vertices(&self) -> Vec<u32> {
            let mut x = Vec::with_capacity(2);
            x.push(self.begin);
            x.push(self.begin + self.lenght);
            x
        }
    }


    #[test]
    fn ctor() {
        ChunkManager::<u32>::new();
    }

    #[test]
    fn add() {
        let mut mgr = ChunkManager::<u32>::new();
        assert_eq!(mgr.free.len(), 0);

        assert_eq!(mgr.vertices.len(), 0);
        assert_eq!(mgr.indices.len(), 0);

        let a = Segment {
            begin: 0,
            lenght: 10,
        };
        let b = Segment {
            begin: 10,
            lenght: 10,
        };
        let c = Segment {
            begin: 0,
            lenght: 20,
        };


        mgr.add_object(&a);
        assert_eq!(mgr.vertices().len(), 2);
        assert_eq!(mgr.indices().len(), 2);

        mgr.add_object(&b);
        assert_eq!(mgr.vertices().len(), 3);
        assert_eq!(mgr.indices().len(), 4);

        mgr.add_object(&c);
        assert_eq!(mgr.vertices().len(), 3);
        assert_eq!(mgr.indices().len(), 6);

        // nothing changes if we repeat

        mgr.add_object(&c);
        assert_eq!(mgr.vertices().len(), 3);
        assert_eq!(mgr.indices().len(), 6);

        mgr.add_object(&b);
        assert_eq!(mgr.vertices().len(), 3);
        assert_eq!(mgr.indices().len(), 6);

        mgr.add_object(&a);
        assert_eq!(mgr.vertices().len(), 3);
        assert_eq!(mgr.indices().len(), 6);
        assert_eq!(mgr.free.len(), 0);
    }

    #[test]
    fn batch() {
        let mut v = Vec::new();

        let a = Segment {
            begin: 0,
            lenght: 10,
        };
        let b = Segment {
            begin: 10,
            lenght: 10,
        };
        let c = Segment {
            begin: 0,
            lenght: 20,
        };
        let d = Segment {
            begin: 0,
            lenght: 30,
        };

        v.push(a);
        v.push(b);
        v.push(c);
        v.push(d);

        let mut mgr = ChunkManager::<u32>::new();

        mgr.add_batch(&v);
        assert_eq!(mgr.vertices().len(), 4);
        assert_eq!(mgr.indices().len(), 8);
        assert_eq!(mgr.free.len(), 0);

        v.remove(3);

        mgr.add_batch(&v);

        assert_eq!(mgr.vertices().len(), 4);
        assert_eq!(mgr.indices().len(), 8);

        assert_eq!(mgr.free.len(), 1);
    }
}
