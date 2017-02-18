
use renderer::culing::quadtree;
use image;
use cgmath::{Matrix4, Vector4};


#[derive(Debug, PartialEq)]
enum RelPos {
    Less,
    In,
    Greather,
    Back,
}

pub type Patch = quadtree::Patch;

/// Line Of Sight computation,
/// given:
/// * a set of four coordinates
/// * the `self.height_map` map
/// * model, view and perspetive matrices (pvm)
/// computes the set of patches which are, at worst, partially visible
pub struct Los {
    patches: Vec<Patch>,
    height_map: image::RgbImage,
    last_matrix: Matrix4<f32>,
    last_precission: u32,
}

impl Los {
    /// generate new line of sight tracking object
    /// here is the thing, we could store the texture in here, and bind
    /// on new object... but then we have a borrowed texture for the whole program
    /// execution. For this reason, I guess I will copy the buffer localy....
    pub fn new(depth: &image::RgbImage) -> Los {
        use cgmath::Zero;
        Los {
            patches: Vec::<Patch>::new(),
            height_map: depth.clone(),
            last_matrix: Matrix4::zero(),
            last_precission: 0,
        }
    }

    pub fn get_patches(&self) -> &Vec<Patch> {
        &self.patches
    }

    pub fn update_view(&mut self, precision: u32, pvm: &Matrix4<f32>) {
        use renderer::culing::quadtree::{test, TestResult};

        if self.last_matrix == *pvm && self.last_precission == precision {
            return;
        }
        self.last_matrix = pvm.clone();
        self.last_precission = precision;
        //println!(" ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ ");
        //println!("chunk_size {}", precision);
        //println!(" ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ ");

        let (size_x, size_z) = self.height_map.dimensions();

        let tree = Patch::new((0, 0), (size_x - 1, size_z - 1));
        self.patches = test(precision,
                            tree,
                            &|p| {

            //println!("{:?}", p);
            let (a, b, c, d) = p.get_corners();

            let res = [check_voxel(a, pvm, &self.height_map),
                       check_voxel(b, pvm, &self.height_map),
                       check_voxel(c, pvm, &self.height_map),
                       check_voxel(d, pvm, &self.height_map)];

            // println!(" {:?}\t{:?}", p, res);


            // if all are behind, discard
            if res.iter().all(|x| match *x {
                (RelPos::Back, _) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }

            //  ~~~~~~~~~~~ behind camera ~~~~~~~~~~~~~~~~~~~~~~~~~
            if res.iter().all(|x| match *x {
                (_, RelPos::Back) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }

            //  ~~~~~~~~~~~ same side? ~~~~~~~~~~~~~~~~~~~~~~~~~~~
            if res.iter().all(|x| match *x {
                (RelPos::Less, _) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }
            if res.iter().all(|x| match *x {
                (_, RelPos::Less) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }
            if res.iter().all(|x| match *x {
                (RelPos::Greather, _) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }

            if res.iter().all(|x| match *x {
                (_, RelPos::Greather) => true,
                _ => false,
            }) {
                return TestResult::Discard;
            }

            //  ~~~~~~~~~~~ all inside? ~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //  we could take, but is better if let it reach the base case
            //  this way we get chunks of the very same size
            // if res.iter().all(|x:&(RelPos, RelPos)|{
            //     match *x{
            //         (RelPos::In, RelPos::In) => true,
            //         _ => false,
            //     }
            // }){
            //     //println!("Take {:?} {:?}", p, res);
            //     return TestResult::Take;
            // }

            TestResult::Refine
        });
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.height_map.dimensions()
    }
}

/// check whenever a 2,5D coordinate is inside of the view
///http://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/projection-matrix-GPU-rendering-pipeline-clipping
fn check_voxel(corner: (u32, u32),
               pvm: &Matrix4<f32>,
               height_map: &image::RgbImage)
               -> (RelPos, RelPos) {
    use image::Pixel;

    let (x, z) = corner;

    let pixel = height_map.get_pixel(x, z);
    let h = (pixel.channels()[0] as f32 / 5.0).trunc();
    let v = Vector4::new(x as f32, h, z as f32, 1.0);

    let pos = pvm * v;

    if pos.w <= 0.0 {
        return (RelPos::Back, RelPos::Back);
    }
    let a = pos.x / pos.w;
    let b = pos.y / pos.w;
    let c = pos.z / pos.w;
    //println!("{:?} ({},{})", corner, a, b);

    if c <= 0.0 {
        return (RelPos::Back, RelPos::Back);
    }
    if c > 1.0 {
        return (RelPos::Back, RelPos::Back);
    }

    let mut i = RelPos::In;
    let mut j = RelPos::In;
    if a < -1.0 {
        i = RelPos::Less;
    }
    if b < -1.0 {
        j = RelPos::Less;
    }
    if a > 1.0 {
        i = RelPos::Greather;
    }
    if b > 1.0 {
        j = RelPos::Greather;
    }

    (i, j)
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Tests:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {
    use super::Los;
    use cgmath::{Point3, Vector3, Matrix4, deg, perspective};

    #[test]
    fn los_ctor() {
        let height_map = world::image_atlas::load_rgb("assets/test.png");
        Los::new(&height_map);
    }


    use world;
    use std::fmt::Debug;
    use time;

    fn print<T>(v: &Vec<T>)
        where T: Debug
    {
        println!("vector contains:");
        v.iter()
            .map(|elem| {
                println!("\t{:?}", elem);
            })
            .last();
    }


    fn load_pvm(h: u32, w: u32) -> Matrix4<f32> {
        let size_x = h as f32;
        let size_z = w as f32;

        let view = Matrix4::look_at(Point3::new(0.0, 75.0, -110.0),
                                    Point3::new(0.0, 0.0, 0.0),
                                    Vector3::new(0.0, 1.0, 0.0));
        let perspective: Matrix4<f32> = perspective(deg(45.0), 1920.0 / 1080.0, 5.0, 1100.0);
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

        println!("load height_map map ");
        // read height_map map
        let height_map = world::image_atlas::load_rgb("assets/test.png");
        let mut los = Los::new(&height_map); // translations for the instances
        let (size_x, size_z) = height_map.dimensions();

        let pvm = load_pvm(size_x, size_z);

        println!("test ");

        let start_time = time::precise_time_s();
        los.update_view(100, &pvm);
        let end_time = time::precise_time_s();

        print(los.get_patches());
        println!(" contains: {} patches, in: {:?}",
                 los.get_patches().len(),
                 end_time - start_time);
    }
}
