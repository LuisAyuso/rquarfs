
extern crate glium;
extern crate image;
extern crate threadpool;

use std::fs;
use std::result;
use std::io;
use std::path::PathBuf;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// Textures module, the idea is to provide an interface
// to deal with io in an efficient manner, load textures, create an atlas.
// return ONE texture to be used by the program and the paramenters needed
// to use a shader on it


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub fn load_images_rgba(path : &PathBuf) -> Vec<image::RgbaImage>
{
	let path = fs::canonicalize(&path).unwrap();
	print!("load image: {:?}\n", path);

	let count = fs::read_dir(&path).unwrap().count();
    let mut images = Vec::with_capacity(count);

    // iterate over images:
	use std::fs;
	for entry in fs::read_dir(&path).unwrap() {
        let dir = entry.unwrap();
        let image = image::open(dir.path()).unwrap();
        images.push(image.to_rgba());
        print!("load {:?}\n", dir.file_name());
	}

    images
}


pub fn load_textures<F: glium::backend::Facade> (display: &F,  set_name : &str) -> Vec<glium::texture::Texture2d>
{
	let mut path = fs::canonicalize(".").unwrap();
	path.push("assets");
	path.push(set_name);
	print!("load textures: {:?}\n", path);

    let images = load_images_rgba(&path);
    let mut textures = Vec::new();

    // iterate over textures:
	use std::fs;
	for image in images {
		let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();
        textures.push(texture)
	}

    textures
}


pub struct Atlas{
    count: usize,
    tex_w: usize,
    tex_h: usize,
    side: usize,

    image: image::RgbaImage
}

impl Atlas{

    fn new( count: usize, tex_w: usize, tex_h: usize, side: usize, image: image::RgbaImage) 
        -> Atlas
    {
         Atlas{
            count: count,
            tex_w: tex_w,
            tex_h: tex_h,
            side: side,
            image: image,
        }
    }

    fn from_file(path: &PathBuf){
    }

    fn save(&self, path: &PathBuf, name: &str) 
        -> Result<(), io::Error>
    {
        assert!(self.image.dimensions().0 as usize == self.tex_w * self.side);
        assert!(self.image.dimensions().1 as usize == self.tex_h * self.side);

        // craft a name we can undestand
        let file_name = format!("{}.{}_{}x{}_{}x{}.atlas.png", name, self.count, 
                            self.tex_w, self.tex_h, self.side, self.side);

        let mut file_path = path.clone();
        file_path.push(file_name);

        let folder = file_path.parent();
        fs::create_dir(&folder.unwrap());

        print!("save->{:?}\n", file_path);
        self.image.save(&file_path)
    }
}


// this function will create an atlas with the pictures found in folder
// the folder will be fetch from the assets folder 
// ideally will be cached in assets/cache
pub fn generate_atlas(set_name : &str) -> Atlas
{
	let mut path = fs::canonicalize(".").unwrap();
	path.push("assets");
	path.push(set_name);
	print!("load textures: {:?}\n", path);

	let tex_count = fs::read_dir(&path).unwrap().count();

    // put textures in a square:
    let side = (tex_count as f32).sqrt().ceil() as u32;
    
	print!("textures: {} -> side:{}  \n", tex_count, side);

    let images = load_images_rgba(&path);

    // get texture dimmension and validate that all have same size
    let dimensions = images.iter().map(|x| x.dimensions()).collect::<Vec<(u32, u32)>>();
    let (texture_w, texture_h) = dimensions[0];
    assert!(dimensions.iter().all(|x| x.0 == texture_w && x.1 == texture_h), 
                    "some texture has different size");

    let atlas_width = side*texture_w;
    let atlas_height = side*texture_h;

    // create image,
    let mut atlas_image = image::RgbaImage::new(atlas_width, atlas_height);

    // iterate over cells and fill them with each image
    use image::GenericImage;
    let mut count = 0;
    for i in 0..side {
        for j in 0..side {
            let mut cell = image::SubImage::new(&mut atlas_image, i*texture_w, j*texture_h, texture_w, texture_h);
            cell.copy_from(&images[count], 0,0);

            count+=1;
            if count >= images.len() { break; }
        }
        if count >= images.len() { break; }
    }

    // resize?
	let cache_path = fs::canonicalize("./assets/cache/").unwrap();
    // save to cache. cache is the same as the set_name name
    let atlas = Atlas::new(tex_count, texture_w as usize, texture_h as usize, side as usize, atlas_image);
    assert!(atlas.save(&cache_path, &set_name).is_ok());
    atlas
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


#[cfg(test)]
mod tests 
{

    use super::generate_atlas;

    #[test]
    fn atlas() 
    {
        generate_atlas("test/atlas1");
        generate_atlas("test/atlas2");
//        generate_atlas("test/atlas3");
    }


}
