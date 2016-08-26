
pub type Point = (u32, u32);
pub type Vector = (u32, u32);


//// the idea is the following, this is a virtual KDtree,
//// there is no tree stored in memory, the bounds and geometries
//// are always the same, and space is filled in a continious manner.
//// what we need is a function where, given dimensions of the map
//// returns the set of elements which pass a given test, such test
//// will of course use the geometrical properties of the coordinate
#[derive(Copy, Clone, Debug)]
pub struct Patch{
    p: Point,
    v: Vector,
}

//   (c)                p + v (d)
//    x-------------------x
//    |         |         |
//    |         |         |
//    |---------|---------|
//    |         |         |
//    |         |         |
//    x-------------------x
//  p  (a)                (b)

impl Patch{
    pub fn new(p: Point, v: Vector) -> Patch{
        Patch{
            p: p,
            v: v
        }
    }

    fn split_v(self) -> (Patch, Patch){
        let half = self.v.1 / 2;  // 1 is y
        (Patch{
            p: self.p,
            v: (self.v.0, half),
        },
        Patch{
            p: (self.p.0, self.p.1 + half),
            v: (self.v.0, half),
        }
        )
    }
    fn split_h(self) -> (Patch, Patch){
        let half = self.v.0 / 2;  // 0 is x
        (Patch{
            p: self.p,
            v: (half, self.v.1),
        },
        Patch{
            p: (self.p.0 + half, self.p.1),
            v: (half, self.v.1),
        })
    }

    fn get_corners(self) -> (Point, Point, Point, Point){
        (
            self.p,
            (self.p.0, self.p.1 + self.v.1),
            (self.p.0+ self.v.0, self.p.1),
            (self.p.0+ self.v.0, self.p.1 + self.v.1),
        )
    }
}


/// the test will be applied to every partition of the space,
/// if any of the corners passes the test, it will call recursion with the four partitions
/// if all of the corners passes the test, it will return the shape
/// BC: is possitive in the base case if any of the corners passes the test
#[inline]
pub fn test<Fun>(bc: u32, x: Patch, f:&Fun) -> Vec<Patch>
where Fun: Fn(Point) ->bool
{
    rec_h(bc, x, f)
}

#[derive(PartialEq, Debug)]
enum TestResult{
    All,
    Some,
    None,
}

#[inline]
fn test_corners<Fun> (x: &Patch, f:&Fun) -> TestResult
where Fun: Fn(Point) ->bool{
    let (a,b,c,d) = x.get_corners();
    let ra = f(a);
    let rb = f(b);
    let rc = f(c);
    let rd = f(d);

    if ra && rb && rc && rd {
        return TestResult::All;
    }
    if ra || rb  || rc || rd {
        return TestResult::Some;
    }
    return TestResult::None
}

/// horizontal split
fn rec_h<Fun> (bc: u32, x: Patch, f:&Fun) -> Vec<Patch>
where Fun: Fn(Point) ->bool
{
    if x.v.1 <= bc{
        return vec!(x);
    }

    let(down, up) = x.split_v();
    // test up
    let a = test_corners(&up, f);
    // test down
    let b = test_corners(&down, f);

    //println!("h{:?} {:?}", a, b);
    match (a, b) {
        (TestResult::All, TestResult::All) => return vec!(x),
        (TestResult::All, TestResult::Some) => return add_elem(rec_v(bc, down, f), up),
        (TestResult::Some, TestResult::All) => return add_elem(rec_v(bc, up, f), down),
        (TestResult::Some, TestResult::None) =>  rec_v(bc, up, f),
        (TestResult::None, TestResult::Some) =>  rec_v(bc, down, f),
        (TestResult::Some, TestResult::Some) =>  union(rec_v(bc, down, f),rec_v(bc, up, f)),
        _ =>  Vec::<Patch>::new(),
    }
}

/// vertical split
fn rec_v<Fun> (bc: u32, x: Patch, f:&Fun) -> Vec<Patch>
where Fun: Fn(Point) ->bool
{
    if x.v.0 <= bc{
        return vec!(x);
    }

    let(left, right) = x.split_h();
    // test up
    let a = test_corners(&left, f);
    // test down
    let b = test_corners(&right, f);

    //println!("v{:?} {:?}", a, b);
    match (a, b) {
        (TestResult::All, TestResult::All) => return vec!(x),
        (TestResult::All, TestResult::Some) =>   add_elem(rec_h(bc, right, f), left),
        (TestResult::Some, TestResult::All) =>   add_elem(rec_h(bc, left, f), right),
        (TestResult::Some, TestResult::None) =>  rec_h(bc, left, f),
        (TestResult::None, TestResult::Some) =>  rec_h(bc, right, f),
        (TestResult::Some, TestResult::Some) =>  union(rec_h(bc, left, f),rec_h(bc, right, f)),
        _ =>  Vec::<Patch>::new(),
    }
}

fn add_elem<T> (v: Vec<T>, elem: T) -> Vec<T>{
    let mut res = v;
    res.push(elem);
    res
}

fn union<T> (v1: Vec<T>, v2: Vec<T>) -> Vec<T>{
    let mut res = v1;
    let mut other = v2;
    res.append(&mut other);
    res
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//   test
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::Patch;
    use std::fmt::Debug;

    fn print<T> (v: &Vec<T>)
        where T : Debug
    {
        println!("vector contains:");
        v.iter().map(|elem|{
                println!("\t{:?}", elem);
            }).last();
    }

    #[test]
    fn vertical_split(){
        let x = Patch::new((0,0), (8,8));
        let(down, up) = x.split_v();
        println!("{:?}", down);
        println!("{:?}", up);

        assert_eq!(down.p.0, x.p.0);
        assert_eq!(down.p.1, 0);
        assert_eq!(down.v.0, x.v.0);
        assert_eq!(down.v.1, 4);

        assert_eq!(up.p.0, x.p.0);
        assert_eq!(up.p.1, 4);
        assert_eq!(up.v.0, x.v.0);
        assert_eq!(up.v.1, 4);
    }

    #[test]
    fn horizontal_split(){
        let x = Patch::new((0,0), (8,8));
        let(left, right) = x.split_h();
        println!("{:?}", left);
        println!("{:?}", right);

        assert_eq!(left.p.0, x.p.0);
        assert_eq!(left.p.1, x.p.1);
        assert_eq!(left.v.0, x.v.0/2);
        assert_eq!(left.v.1, x.v.1);

        assert_eq!(right.p.0, x.p.0+4);
        assert_eq!(right.p.1, x.p.1);
        assert_eq!(right.v.0, x.v.0/2);
        assert_eq!(right.v.1, x.v.1);
    }

    #[test]
    fn corners(){
        let x = Patch::new((10,0), (22,8));
        let(a, b, c, d) = x.get_corners();
        assert_eq!(a, (10,0));
        assert_eq!(b, (10,8));
        assert_eq!(c, (32,0));
        assert_eq!(d, (32,8));

        let x = Patch::new((0,0), (8,8));
        let(a, b, c, d) = x.get_corners();
        assert_eq!(a, (0,0));
        assert_eq!(b, (0,8));
        assert_eq!(c, (8,0));
        assert_eq!(d, (8,8));
    }

    #[test]
    fn test(){
        use super::test_corners;
        use super::TestResult;

        let x = Patch::new((10,0), (22,8));
        let ra = test_corners(&x,&|(_,_)|{
         //   println!("test {} {}", x, y);
            true
        });
        assert_eq!(ra, TestResult::All);
        let ra = test_corners(&x,&|(_,y)|{
            y > 0
        });
        assert_eq!(ra, TestResult::Some);
        let ra = test_corners(&x,&|(_,_)|{
            false
        });
        assert_eq!(ra, TestResult::None);
    }

    #[test]
    fn search(){
        println!("check recursion1");
        use super::test;

        let x = Patch::new((0,0), (8,8));
        {
            let res = test(4, x, &|(x,y)|{
                println!("\ttest {} {}", x, y);
                false
            });
            print(&res);
            assert_eq!(res.len(), 0);
        }

        {
            let res = test(4, x, &|(x,y)|{
                println!("\ttest {} {}", x, y);
                true
            });
            print(&res);
            assert_eq!(res.len(), 1);
        }

        {
            let res = test(2, x, &|(x,y)|{
                println!("\ttest {} {} {}", x, y, y > 4);
                y > 4
            });
            print(&res);
            assert_eq!(res.len(), 6);
        }

    }

    use world;
    use cgmath::{Point3, Vector3, Vector4, Matrix4, deg, perspective};


    fn load_pvm(h: u32, w: u32) -> Matrix4<f32>{
        let size_x = h as f32;
        let size_z = w as f32;

		let view = Matrix4::look_at(Point3::new(0.0, 75.0, -110.0),
                                    Point3::new(0.0, 0.0, -0.0), 
                                    Vector3::new(0.0, 1.0, 0.0));
        let perspective: Matrix4<f32> = perspective(deg(45.0), 1.0, 2.0, 100.0);
        let model = Matrix4::from_translation(Vector3::new(-(size_x / 2.0), 0.0, -(size_z / 2.0)));
        perspective * view * model
    }

    #[test]
    fn pvm_checks() {
        use image::Pixel;
  
        print!("load height map \n");
        // read height map 
        let height = world::textures::load_rgb("assets/height.jpg");
        let height_dimensions = height.dimensions();

        // translations for the instances
        let size_x = height_dimensions.0;
        let size_z = height_dimensions.1;

        let pvm = load_pvm(size_x, size_z);

        print!("test \n");

        let tree = Patch::new((0,0), (size_x as u32 -1, size_z as u32 -1));
        let res = super::test(100, tree, &|(x, z)|{
            let pixel = height.get_pixel(x,z);
            let components = pixel.channels();
            println!("({},{},{})", x, components[0], z);
            let v = Vector4::new(x as f32, components[0] as f32, z as f32, 1.0);

            let pos = pvm * v;
            println!("({:?} => {:?})", v, pos);

            let res = pos.x >= 0.0 && pos.x <= 1.0 && pos.y >= 0.0 && pos.y <= 1.0;
            println!("{}", res);

            res
        });
        print(&res);
        // four corner out of the grid
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn pvm_checks() {
        use image::Pixel;
  
        print!("load height map \n");
        // read height map 
        let height = world::textures::load_rgb("assets/height.jpg");
        let height_dimensions = height.dimensions();

        // translations for the instances
        let size_x = height_dimensions.0;
        let size_z = height_dimensions.1;

        let pvm = load_pvm(size_x, size_z);
        let frustum = cgmath::Frustum::from_matrix4(pvm);

        print!("test \n");

        let tree = Patch::new((0,0), (size_x as u32 -1, size_z as u32 -1));
        let res = super::test(100, tree, &|(x, z)|{
            let pixel = height.get_pixel(x,z);
            let components = pixel.channels();
            println!("({},{},{})", x, components[0], z);
            let v = Vector4::new(x as f32, components[0] as f32, z as f32, 1.0);

            let pos = pvm * v;
            println!("({:?} => {:?})", v, pos);

            let res = pos.x >= 0.0 && pos.x <= 1.0 && pos.y >= 0.0 && pos.y <= 1.0;
            println!("{}", res);

            res
        });
        print(&res);
    }
}
