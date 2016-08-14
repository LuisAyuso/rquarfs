extern crate glium;
extern crate image;

// Textures module, the idea is to provide an interface
// to deal with io in an efficient manner, load textures, create an atlas.
// return ONE texture to be used by the program and the paramenters needed
// to use a shader on it

pub fn load_textures<F: glium::backend::Facade> (display: &F,  set : &str) -> Vec<glium::texture::Texture2d>
{
	let mut path = fs::canonicalize(".").unwrap();
	path.push("assets");
	path.push(set);
	print!("load textures: {:?}\n", path);

    let mut textures = Vec::new();

    // iterate over textures:
	use std::fs;
	for entry in fs::read_dir(path).unwrap() {
		let dir = entry.unwrap();
		let image = image::open(dir.path()).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();

		print!(" {:?} -> {}x{}\n", dir.file_name(), image_dimensions.0, image_dimensions.1);
        textures.push(texture)
	}

    textures
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

