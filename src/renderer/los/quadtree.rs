
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
    pub p: Point,
    pub v: Vector,
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

    fn split_v(&self) -> (Patch, Patch){
        let half = self.v.1 / 2;  // 1 is y
        (Patch{
            p: self.p,
            v: (self.v.0, half-1),
        },
        Patch{
            p: (self.p.0, self.p.1 + half),
            v: (self.v.0, half),
        }
        )
    }
    fn split_h(&self) -> (Patch, Patch){
        let half = self.v.0 / 2;  // 0 is x
        (Patch{
            p: self.p,
            v: (half-1, self.v.1),
        },
        Patch{
            p: (self.p.0 + half, self.p.1),
            v: (half, self.v.1),
        })
    }

    pub fn get_corners(&self) -> (Point, Point, Point, Point){
        (
            self.p,
            (self.p.0, self.p.1 + self.v.1),
            (self.p.0+ self.v.0, self.p.1),
            (self.p.0+ self.v.0, self.p.1 + self.v.1),
        )
    }
}

/// functions passed to the test must return what to do with the patch
/// this are the folling options
#[derive(PartialEq, Debug)]
pub enum TestResult{
    Refine,
    Discard,
    Take,
}



/// the test will be applied to every partition of the space,
/// if any of the corners passes the test, it will call recursion with the four partitions
/// if all of the corners passes the test, it will return the shape
/// BC: is possitive in the base case if any of the corners passes the test
#[inline]
pub fn test<Fun>(bc: u32, x: Patch, f:&Fun) -> Vec<Patch>
where Fun: Fn(&Patch) ->TestResult
{
    rec_h(bc, x, f)
}

/// horizontal split
fn rec_h<Fun> (bc: u32, x: Patch, test_f:&Fun) -> Vec<Patch>
where Fun: Fn(&Patch) ->TestResult
{
    if x.v.1 <= bc{
        return vec!(x);
    }

    let(down, up) = x.split_v();
    let a = test_f(&down);
    let b = test_f(&up);

    //println!("h{:?} {:?}", a, b);
    match (a, b) {
        (TestResult::Take, TestResult::Take) => return vec!(x),
        (TestResult::Take, TestResult::Refine) => return add_elem(rec_v(bc, up, test_f), down),
        (TestResult::Refine, TestResult::Take) => return add_elem(rec_v(bc, down, test_f), up),
        (TestResult::Refine, TestResult::Refine) =>  union(rec_v(bc, down, test_f),rec_v(bc, up, test_f)),
        (TestResult::Refine, TestResult::Discard) =>  rec_v(bc, down, test_f),
        (TestResult::Discard, TestResult::Refine) =>  rec_v(bc, up, test_f),
        _ =>  Vec::<Patch>::new(),
    }
}

/// vertical split
fn rec_v<Fun> (bc: u32, x: Patch, test_f:&Fun) -> Vec<Patch>
where Fun: Fn(&Patch) ->TestResult
{
    if x.v.0 <= bc{
        return vec!(x);
    }

    let(left, right) = x.split_h();
    let a = test_f(&left);
    let b = test_f(&right);

    //println!("v{:?} {:?}", a, b);
    match (a, b) {
        (TestResult::Take, TestResult::Take) => return vec!(x),
        (TestResult::Take, TestResult::Refine) =>   add_elem(rec_h(bc, right, test_f), left),
        (TestResult::Refine, TestResult::Take) =>   add_elem(rec_h(bc, left, test_f), right),
        (TestResult::Refine, TestResult::Refine) =>  union(rec_h(bc, left, test_f),rec_h(bc, right, test_f)),
        (TestResult::Refine, TestResult::Discard) =>  rec_h(bc, left, test_f),
        (TestResult::Discard, TestResult::Refine) =>  rec_h(bc, right, test_f),
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
    use super::TestResult;
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
        assert_eq!(down.v.1, 3);

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
        assert_eq!(left.v.0, x.v.0/2-1);
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
//        use super::test_f;
//        use super::TestResult;
//
//        let x = Patch::new((10,0), (22,8));
//        let ra = test_f(&x,&|(_,_)|{
//         //   println!("test {} {}", x, y);
//            true
//        });
//        assert_eq!(ra, TestResult::Take);
//        let ra = test_f(&x,&|(_,y)|{
//            y > 0
//        });
//        assert_eq!(ra, TestResult::Refine);
//        let ra = test_f(&x,&|(_,_)|{
//            false
//        });
//        assert_eq!(ra, TestResult::Discard);
    }

    #[test]
    fn search(){
        println!("check recursion1");
        use super::test;

        let x = Patch::new((0,0), (8,8));
        {
            let res = test(4, x, &|p|{
                println!("\ttest {:?} ", p);
                TestResult::Discard
            });
            print(&res);
            assert_eq!(res.len(), 0);
        }

        {
            let res = test(4, x, &|p|{
                println!("\ttest {:?} ", p);
                TestResult::Take
            });
            print(&res);
            assert_eq!(res.len(), 1);
        }

        {
            let res = test(2, x, &|p|{
                println!("\ttest {:?} ", p);
                TestResult::Refine
            });
            print(&res);
            assert_eq!(res.len(), 4*4);
        }

        {
            let res = test(4, x, &|p|{
                println!("\ttest {:?} ", p);
                TestResult::Refine
            });
            print(&res);
            assert_eq!(res.len(), 2*2);
        }
    }
}
