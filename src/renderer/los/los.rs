
use renderer::los::quadtree;
use image;
use cgmath::{Matrix4, Vector4};


pub type Patch = quadtree::Patch;

/// Line Of Sight computation,
/// given:
/// * a set of four coordinates
/// * the self.height_map map
/// * model, view and perspetive matrices (pvm)
/// computes the set of patches which are, at worst, partially visible
pub struct Los{
    patches: Vec<Patch>,
    height_map: image::RgbImage,
}

impl Los{

    /// generate new line of sight tracking object
    /// here is the thing, we could store the texture in here, and bind
    /// on new object... but then we have a borrowed texture for the whole program
    /// execution. For this reason, I guess I will copy the buffer localy....
    pub fn new(depth: &image::RgbImage) -> Los{
        Los{
            patches: Vec::<Patch>::new(),
            height_map: depth.clone(),
        }
    }

    pub fn get_patches(&self) -> &Vec<Patch>{
        &self.patches
    }

    pub fn update_view(&mut self, precision: u32, pvm: &Matrix4<f32>) {
        use renderer::los::quadtree::{test, TestResult};

        let (size_x, size_z) = self.height_map.dimensions();

        let tree = Patch::new((0,0), (size_x-1, size_z-1));
        self.patches = test(200, tree, &|p|{

            //println!("test {:?}", p);
            let (a,b,c,d) = p.get_corners();

            let res = [check_voxel(a, &pvm, &self.height_map),
            check_voxel(b, &pvm, &self.height_map),
            check_voxel(c, &pvm, &self.height_map),
            check_voxel(d, &pvm, &self.height_map),];

            let mut result = TestResult::Refine;

            //   -----------------
            //   |   x      x    |
            //   |               |
            //   |   x      x    |
            //   -----------------
            // if all in, Take it
            if res.iter().fold(true, |flag, &elem| {
                flag && (elem.0 >= -1.0 && elem.0 <= 1.0) &&
                (elem.1 >= -1.0 && elem.1 <= 1.0) 
            }) { result =  TestResult::Take }

            //         --------------
            //       x |       x    |
            //         |            |
            //       x |       x    |
            //         --------------
            // if any in, Refine 
            else if res.iter().fold(false, |flag, &elem| {
                flag || (elem.0 >= -1.0 && elem.0 <= 1.0) ||
                (elem.1 >= -1.0 && elem.1 <= 1.0) 
            }) { result =  TestResult::Refine }

            //     --------------
            //  x  |            |  x
            //     |            |
            //  x  |            |  x
            //     --------------
            // if both sides (X): Refine
            else if both_sides( &res) {
                result = TestResult::Refine;
            } 

            //         --------------
            //  x    x |            |
            //         |            |
            //  x    x |            |
            //         --------------
            // are on the same side (X): DISCARD
            else if res.iter().fold(true, |flag, &elem| {
                flag && (elem.0 < -1.0 || elem.0 > 1.0)
            }) { result =  TestResult::Discard }

            // are on the same side (Y): DISCARD
            else if res.iter().fold(true, |flag, &elem| {
                flag && (elem.1 < -1.0 || elem.1 > 1.0)
            }) { result =  TestResult::Discard }

            //println!("  {:?}: {:?}",res, result);
            //println!("   {:?}", result);
            result
        });
    }
}

/// check whenever a 2,5D coordinate is inside of the view
fn check_voxel(corner: (u32, u32), pvm: &Matrix4<f32>, height_map: &image::RgbImage) -> (f32, f32){
    use std::cmp;
    use image::Pixel;

    let (x,z) = corner;

    let pixel = height_map.get_pixel(x,z);
    let components = pixel.channels();
    //println!("({},{},{})", x, components[0], z);
    let v = Vector4::new(x as f32, components[0] as f32, z as f32, 1.0);

    let pos = pvm * v;
    let a = pos.x;
    let b = pos.y;

    // println!("{:?} => ({},{})", v, a, b);
    (a,b)
}

/// check whenever the patch overflows overflows both sides, (patch is wider than view and we need 
/// to refine
fn both_sides(v: &[(f32, f32); 4]) -> bool {

    let scoped = v.iter().map(|p|{
        let a = p.0.min(1.0).max(-1.0);
        let b = p.1.min(1.0).max(-1.0);
        assert!(a == -1.0 || a == 1.0);
        assert!(b == -1.0 || b == 1.0);
        (a as i8,b as i8)
    });

    let res = scoped.fold((0,0), |acum, p|{
        (acum.0 + p.0, acum.1 + p.1)
    });

    let bothsides_x = res.0 != 4 && res.0 != -4;
    let bothsides_y = res.1 != 4 && res.1 != -4;
   // println!(" bothsides: ({},{}), {} {}", res.0, res.1, bothsides_x, bothsides_y);
    bothsides_x || bothsides_y
}




// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Tests:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  
#[cfg(test)]
mod tests {
    use super::Los; 
    use cgmath::{Point3, Vector3, Vector4, Matrix4, deg, perspective};
    use super::check_voxel;
    use super::both_sides;

    #[test]
    fn los_ctor()
    {   
        let height_map = world::textures::load_rgb("assets/height.jpg");
        Los::new(&height_map);
    }


    use world;
    use renderer::los::quadtree::{Patch, test, TestResult};
    use std::fmt::Debug;
    use time;

    fn print<T> (v: &Vec<T>)
        where T : Debug
    {
        println!("vector contains:");
        v.iter().map(|elem|{
                println!("\t{:?}", elem);
            }).last();
    }


    fn load_pvm(h: u32, w: u32) -> Matrix4<f32>{
        let size_x = h as f32;
        let size_z = w as f32;

		let view = Matrix4::look_at(Point3::new(0.0, 75.0, -110.0),
                                    Point3::new(0.0, 0.0, 0.0), 
                                    Vector3::new(0.0, 1.0, 0.0));
        let perspective: Matrix4<f32> = perspective(deg(45.0), 1920.0/1080.0, 5.0, 1100.0);
        let model = Matrix4::from_translation(Vector3::new(-(size_x / 2.0), 0.0, -(size_z / 2.0)));
        //let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));

        let pvm = perspective * view * model;
      //  println!("{:?}", perspective);
      //  println!("{:?}", view);
      //  println!("{:?}", model);
      //  println!("{:?}", pvm);
        pvm
    }


    #[test]
    fn pvm_checks() {
  
        print!("load height_map map \n");
        // read height_map map 
        let height_map = world::textures::load_rgb("assets/height.jpg");
        let mut los = Los::new(&height_map);        // translations for the instances
        let (size_x, size_z) = height_map.dimensions();

        let pvm = load_pvm(size_x, size_z);

        print!("test \n");

        let start_time = time::precise_time_s();
        los.update_view(100, &pvm);
        let end_time = time::precise_time_s();

        print(los.get_patches());
        println!(" contains: {} patches, in: {:?}", los.get_patches().len(), end_time-start_time);
    }
}
