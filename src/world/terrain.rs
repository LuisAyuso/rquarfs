
use glium;
use context::Program;
use context::DrawItem;



// the idea here is to create a tessellation terrain,
// lets start with a grid.
pub struct Terrain{
 //   prg: glium::program::Program
}


impl Terrain{

    fn with_shader(program :glium::program::Program)-> Terrain{
        Terrain{
//            prg: program,
        }
    }

}




//impl Program for Terrain {
//    fn get_program(&self) -> &glium::program::Program {
//        &self.prg
//    }
//    fn with_tess(&self) -> bool{
//        self.prg.has_tessellation_shaders()
//    }
//}



#[cfg(test)]
mod tests {

    use super::Terrain;
    use shader;

    #[test]
    fn new(){
        


    }

}
