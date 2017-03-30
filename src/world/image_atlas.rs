extern crate glium;
extern crate image;
extern crate threadpool;
extern crate regex;
extern crate glob;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

use self::regex::Regex;
use self::glob::glob;

use rand;
use rand::distributions::Range;
use rand::distributions::IndependentSample;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// Textures module, the idea is to provide an interface
// to deal with io in an efficient manner, load textures, create an atlas.
// return ONE texture to be used by the program and the paramenters needed
// to use a shader on it
pub fn load_rgb(filename: &str) -> image::RgbImage {
    let path = fs::canonicalize(&filename).unwrap();
    println!("load image: {:?}", path);

    // iterate over images:
    use std::fs;
    let image = image::open(path).unwrap();
    image.to_rgb()
}

pub fn generate_noise(size: (u32, u32)) -> image::RgbImage {

    let (w, h) = size;
    let mut image = image::RgbImage::new(w, h);

    let mut rng = rand::thread_rng();
    let between = Range::new(0u8, 255);
    for pix in image.iter_mut() {
        let r = between.ind_sample(&mut rng);
        *pix = r;
    }

    image
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
pub fn load_images_rgba(path: &PathBuf) -> Vec<image::RgbaImage> {
    let path = fs::canonicalize(&path).unwrap();
    println!("load image: {:?}", path);

    let count = fs::read_dir(&path).unwrap().count();
    let mut images = Vec::with_capacity(count);

    // iterate over images:
    use std::fs;
    for entry in fs::read_dir(&path).unwrap() {
        let dir = entry.unwrap();
        let image = image::open(dir.path()).unwrap();
        images.push(image.to_rgba());
        println!("load {:?}", dir.file_name());
    }

    images
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn get_coords_height(height_map: &image::RgbImage, i: u32, j: u32) -> f32 {
    use image::Pixel;
    let pixel = height_map.get_pixel(i, j);
    (pixel.channels()[0] as f32 / 5.0).trunc()
}

#[allow(dead_code)]
pub fn get_max_neighbour(height_map: &image::RgbImage, i: u32, j: u32) -> f32 {
    // use std::cmp;
    let (max_i, max_j) = height_map.dimensions();

    let kernel = vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

    let res = kernel.iter()
        .map(|pair| {
            let a = i as i32 + pair.0 as i32;
            let b = j as i32 + pair.1 as i32;

            if a < 0 {
                return 256.0;
            }
            if b < 0 {
                return 256.0;
            }
            if a as u32 >= max_i {
                return 256.0;
            }
            if b as u32 >= max_j {
                return 256.0;
            }

            get_coords_height(height_map, a as u32, b as u32)
        })
        .fold(256.0, |acc: f32, x: f32| acc.min(x));

    let current = get_coords_height(height_map, i, j);

    current - res
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
pub struct Atlas {
    pub count: usize,
    pub tex_w: usize,
    pub tex_h: usize,
    pub side: usize,

    pub image: image::RgbaImage,
}

impl Atlas {
    #[allow(dead_code)]
    pub fn new(count: usize,
               tex_w: usize,
               tex_h: usize,
               side: usize,
               image: image::RgbaImage)
               -> Atlas {
        Atlas {
            count: count,
            tex_w: tex_w,
            tex_h: tex_h,
            side: side,
            image: image,
        }
    }

    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Atlas, io::Error> {

        let file_path = fs::canonicalize(path).unwrap();
        let filename = file_path.file_name().unwrap().to_str().unwrap_or("");

        let re = Regex::new(r"\w+\.(\d+)_(\d+)x(\d+)_(\d+)x(\d+)\.atlas.\w+").unwrap();
        match re.captures(filename) {
            Some(cap) => {
                println!("load: {} textures of size {}x{} in grid: {}x{}",
                         cap.get(1).unwrap().as_str(),
                         cap.get(2).unwrap().as_str(),
                         cap.get(3).unwrap().as_str(),
                         cap.get(4).unwrap().as_str(),
                         cap.get(5).unwrap().as_str());

                let count = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let w = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let h = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let side = cap.get(4).unwrap().as_str().parse::<usize>().unwrap();

                let image = image::open(file_path.to_str().unwrap());
                assert!(image.is_ok());
                Ok(Atlas::new(count, w, h, side, image.unwrap().to_rgba()))

            }
            _ => Err(Error::new(ErrorKind::Other, "oh no!")),
        }
    }

    #[allow(dead_code)]
    pub fn save(&self, path: &PathBuf, name: &str) -> Result<(), io::Error> {
        assert!(self.image.dimensions().0 as usize == self.tex_w * self.side);
        assert!(self.image.dimensions().1 as usize == self.tex_h * self.side);

        // craft a name we can undestand
        let file_name = format!("{}.{}_{}x{}_{}x{}.atlas.png",
                                name,
                                self.count,
                                self.tex_w,
                                self.tex_h,
                                self.side,
                                self.side);

        let mut file_path = path.clone();
        file_path.push(file_name);

        let folder = file_path.parent();
        let _ = fs::create_dir(&folder.unwrap());

        println!("save->{:?}", file_path);
        self.image.save(&file_path)
    }
} // Atlas

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// this function will create an atlas with the pictures found in folder
/// the folder will be fetch from the assets folder
/// ideally will be cached in assets/cache
#[allow(dead_code)]
pub fn generate_atlas(set_name: &str) -> Result<Atlas, io::Error> {
    let mut path = fs::canonicalize(".").unwrap();
    path.push("assets");
    path.push(set_name);
    println!("load textures: {:?}", path);

    let tex_count = fs::read_dir(&path).unwrap().count();

    // put textures in a square:
    let side = (tex_count as f32).sqrt().ceil() as u32;

    println!("textures: {} -> side:{}  ", tex_count, side);

    let images = load_images_rgba(&path);

    // get texture dimmension and validate that all have same size
    let dimensions = images.iter().map(|x| x.dimensions()).collect::<Vec<(u32, u32)>>();
    let (texture_w, texture_h) = dimensions[0];
    assert!(dimensions.iter().all(|x| x.0 == texture_w && x.1 == texture_h),
            "some texture has different size");

    let atlas_width = side * texture_w;
    let atlas_height = side * texture_h;

    // create image,
    let mut atlas_image = image::RgbaImage::new(atlas_width, atlas_height);

    // iterate over cells and fill them with each image
    use image::GenericImage;
    let mut count = 0;
    for i in 0..side {
        for j in 0..side {
            let mut cell = image::SubImage::new(&mut atlas_image,
                                                i * texture_w,
                                                j * texture_h,
                                                texture_w,
                                                texture_h);
            cell.copy_from(&images[count], 0, 0);

            count += 1;
            if count >= images.len() {
                break;
            }
        }
        if count >= images.len() {
            break;
        }
    }

    // resize? i readed somewhere about power of 2 mipmaps, this might help.



    let cache_path = fs::canonicalize("./assets/cache/").unwrap();
    // save to cache. cache is the same as the set_name name
    let atlas = Atlas::new(tex_count,
                           texture_w as usize,
                           texture_h as usize,
                           side as usize,
                           atlas_image);
    assert!(atlas.save(&cache_path, &set_name).is_ok());
    Ok(atlas)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
pub fn load_atlas(set_name: &str) -> Result<Atlas, io::Error> {
    match glob(format!("./assets/cache/{}*", set_name).as_str()) {
        Ok(mut m) => {
            match m.next() {
                Some(path) => Atlas::from_file(path.unwrap().to_str().unwrap_or("")),
                None => generate_atlas(set_name),
            }
        }
        Err(_) => Err(Error::new(ErrorKind::Other, "oh no!")),
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Copy, Clone)]
pub struct MeshPoint {
    position: (f32, f32, f32),
}
implement_vertex!(MeshPoint, position);

impl PartialEq for MeshPoint {
    fn eq(&self, other: &MeshPoint) -> bool {
        self.position.0 == other.position.0 && self.position.1 == other.position.1 &&
        self.position.2 == other.position.2
    }
}

// TODO: - mesh is not complete, what if step does not divide the side?
//       - use indices, this can turn to be a pretty damm big mesh
#[allow(dead_code)]
pub fn to_mesh(step: u32, height_map: &image::RgbImage) -> Vec<MeshPoint> {
    let mut list = Vec::new();

    let (max_x, max_y) = height_map.dimensions();
    let (max_i, max_j) = (max_x / step, max_y / step);

    for i in 0..max_i - 1 {
        for j in 0..max_j - 1 {

            let a = (i * step, j * step);
            let b = ((i + 1) * step, (j) * step);
            let c = ((i) * step, (j + 1) * step);
            let d = ((i + 1) * step, (j + 1) * step);

            list.push(MeshPoint {
                position: (a.0 as f32, get_coords_height(height_map, a.0, a.1) as f32, a.1 as f32),
            });
            list.push(MeshPoint {
                position: (c.0 as f32, get_coords_height(height_map, c.0, c.1) as f32, c.1 as f32),
            });
            list.push(MeshPoint {
                position: (b.0 as f32, get_coords_height(height_map, b.0, b.1) as f32, b.1 as f32),
            });

            list.push(MeshPoint {
                position: (c.0 as f32, get_coords_height(height_map, c.0, c.1) as f32, c.1 as f32),
            });
            list.push(MeshPoint {
                position: (d.0 as f32, get_coords_height(height_map, d.0, d.1) as f32, d.1 as f32),
            });
            list.push(MeshPoint {
                position: (b.0 as f32, get_coords_height(height_map, b.0, b.1) as f32, b.1 as f32),
            });

            assert!(i * step < max_x);
            assert!(j * step < max_y);

            if i == 0 {
                list.push(MeshPoint {
                    position: (c.0 as f32,
                               get_coords_height(height_map, c.0, c.1) as f32,
                               c.1 as f32),
                });
                list.push(MeshPoint {
                    position: (a.0 as f32,
                               get_coords_height(height_map, a.0, a.1) as f32,
                               a.1 as f32),
                });
                list.push(MeshPoint { position: (c.0 as f32, 0.0, c.1 as f32) });

                list.push(MeshPoint {
                    position: (a.0 as f32,
                               get_coords_height(height_map, a.0, a.1) as f32,
                               a.1 as f32),
                });
                list.push(MeshPoint { position: (a.0 as f32, 0.0, a.1 as f32) });
                list.push(MeshPoint { position: (c.0 as f32, 0.0, c.1 as f32) });
            }
            if i == max_i - 2 {
                list.push(MeshPoint { position: (b.0 as f32, 0.0, b.1 as f32) });
                list.push(MeshPoint {
                    position: (b.0 as f32,
                               get_coords_height(height_map, b.0, b.1) as f32,
                               b.1 as f32),
                });
                list.push(MeshPoint {
                    position: (d.0 as f32,
                               get_coords_height(height_map, d.0, d.1) as f32,
                               d.1 as f32),
                });

                list.push(MeshPoint {
                    position: (d.0 as f32,
                               get_coords_height(height_map, d.0, d.1) as f32,
                               d.1 as f32),
                });
                list.push(MeshPoint { position: (d.0 as f32, 0.0, d.1 as f32) });
                list.push(MeshPoint { position: (b.0 as f32, 0.0, b.1 as f32) });
            }
            if j == 0 {
                list.push(MeshPoint {
                    position: (a.0 as f32,
                               get_coords_height(height_map, a.0, a.1) as f32,
                               a.1 as f32),
                });
                list.push(MeshPoint {
                    position: (b.0 as f32,
                               get_coords_height(height_map, b.0, b.1) as f32,
                               b.1 as f32),
                });
                list.push(MeshPoint { position: (a.0 as f32, 0.0, a.1 as f32) });

                list.push(MeshPoint {
                    position: (b.0 as f32,
                               get_coords_height(height_map, b.0, b.1) as f32,
                               b.1 as f32),
                });
                list.push(MeshPoint { position: (b.0 as f32, 0.0, b.1 as f32) });
                list.push(MeshPoint { position: (a.0 as f32, 0.0, a.1 as f32) });
            }
            if j == max_j - 2 {
                list.push(MeshPoint {
                    position: (c.0 as f32,
                               get_coords_height(height_map, c.0, c.1) as f32,
                               c.1 as f32),
                });
                list.push(MeshPoint { position: (c.0 as f32, 0.0, c.1 as f32) });
                list.push(MeshPoint {
                    position: (d.0 as f32,
                               get_coords_height(height_map, d.0, d.1) as f32,
                               d.1 as f32),
                });

                list.push(MeshPoint {
                    position: (d.0 as f32,
                               get_coords_height(height_map, d.0, d.1) as f32,
                               d.1 as f32),
                });
                list.push(MeshPoint { position: (c.0 as f32, 0.0, c.1 as f32) });
                list.push(MeshPoint { position: (d.0 as f32, 0.0, d.1 as f32) });
            }
        }
    }


    let a = (max_i as f32 - 1.0) * step as f32;
    let b = (max_j as f32 - 1.0) * step as f32;
    let d = -100.0;


    // inverted piramid, only 4 triangles close the volume
    list.push(MeshPoint { position: (0.0, 0.0, 0.0) });
    list.push(MeshPoint { position: (a, 0.0, 0.0) });
    list.push(MeshPoint { position: (a / 2.0, d, b / 2.0) }); // check

    list.push(MeshPoint { position: (0.0, 0.0, 0.0) });
    list.push(MeshPoint { position: (a / 2.0, d, b / 2.0) });
    list.push(MeshPoint { position: (0.0, 0.0, b) });

    list.push(MeshPoint { position: (0.0, 0.0, b) });
    list.push(MeshPoint { position: (a / 2.0, d, b / 2.0) });
    list.push(MeshPoint { position: (a, 0.0, b) });

    list.push(MeshPoint { position: (a, 0.0, b) });
    list.push(MeshPoint { position: (a / 2.0, d, b / 2.0) });
    list.push(MeshPoint { position: (a, 0.0, 0.0) });

    list
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


#[cfg(test)]
mod tests {

    //use super::generate_atlas;
    //use super::load_atlas;
    //use super::Atlas;

    //    #[test]
    //    fn test1() {
    //        assert!(generate_atlas("test/atlas1").is_ok());
    //        assert!(Atlas::from_file("./assets/cache/test/atlas1.1_750x750_1x1.atlas.png").is_ok());
    //        assert!(load_atlas("test/atlas1").is_ok());
    //    }
    //
    //    #[test]
    //    fn test2() {
    //        assert!(generate_atlas("test/atlas2").is_ok());
    //        assert!(load_atlas("test/atlas2").is_ok());
    //        assert!(Atlas::from_file("./assets/cache/test/atlas2.2_750x750_2x2.atlas.png").is_ok());
    //    }
    //
    //    #[test]
    //    fn test3() {
    //        assert!(generate_atlas("test/atlas3").is_ok());
    //        assert!(Atlas::from_file("./assets/cache/test/atlas3.25_750x750_5x5.atlas.png").is_ok());
    //        assert!(load_atlas("test/atlas3").is_ok());
    //    }

    use super::load_rgb;
    use super::to_mesh;

    #[test]
    fn get_mesh() {
        let map = load_rgb("assets/pico.png");
        let _ = to_mesh(10, &map);
    }
}
