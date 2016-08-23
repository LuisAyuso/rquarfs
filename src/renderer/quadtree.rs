use cgmath::{Point2, Vector2};


type Point = Point2<f32>;
type Vector = Vector2<f32>;



// the idea is the following, this is a virtual KDtree,
// there is no tree stored in memory, the bounds and geometries
// are always the same, and space is filled in a continious manner.
// what we need is a function where, given dimensions of the map
// returns the set of elements which pass a given test, such test
// will of course use the geometrical properties of the coordinate

//                    origin + length
//    x-------------------x
//    |                   |
//    |                   |
//    |                   |
//    |                   |
//    x-------------------x
//  origin

struct VirtualTree{
    origin: Point,
    lenght: Vector,
}

impl VirtualTree{

    pub fn new( origin: &Point, lenght: &Vector) -> VirtualTree
    {
        VirtualTree{
            origin: origin.clone(),
            lenght: lenght.clone(),
        }
    }

}



#[cfg(test)]
mod tests {
    use super::VirtualTree;
    use super::{Point,Vector};

    #[cfg(test)]
    fn test1(){
        VirtualTree::new(&Point::new(0.0,0.0), &Vector::new(1.0, 1.0));
    }

}
