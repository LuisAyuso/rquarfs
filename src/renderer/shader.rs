use glium;
use std::io;
use std::io::Read;
use std::path;
use std::fs;
use std::time;



fn get_path_to_shader(name: &str) -> Result<path::PathBuf, io::Error> {

    let mut path = try!(fs::canonicalize("."));
    path.push("shaders");
    path.push(format!("{}{}", name, ".glsl"));
    //println!("load shader: {:?}", path);
    Ok(path)
}


/// load shader from path into a string. this is a file into buffer read
fn read_shader(name: &str) -> Result<String, io::Error> {
    use std::fs::File;

    let path = try!(get_path_to_shader(name));

    let mut f = try!(File::open(path));

    let mut shader_buff = String::new();
    let _ = f.read_to_string(&mut shader_buff);
    Ok(shader_buff)
}


fn print_err(err:  glium::program::ProgramCreationError){
    match err{
        glium::program::ProgramCreationError::CompilationError(x) => println!("{}", x),
        glium::program::ProgramCreationError::LinkingError(y) => println!("{}", y),
        z => println!("{:?}", z),
    }
}

/// loads both shaders and compiles program
fn load_program<F: glium::backend::Facade>(display: &F, vs_name: &str, fs_name: &str) 
        -> Option<glium::Program>
{
    // load vs
    let vs = read_shader(vs_name);
    if vs.is_err(){
        println!("{:?}", vs);
        return None;
    }
    // load fs
    let fs = read_shader(fs_name);
    if fs.is_err(){
        println!("{:?}", fs);
        return None;
    }

    // compile
    let prog = glium::Program::from_source(display, &vs.unwrap(), &fs.unwrap(), None);
        //compile_program(display, &vs.unwrap(), &fs.unwrap());
    if let Err(x) = prog{
        print_err(x); 
        return None; 
    }
    Some(prog.unwrap())
}

/// loads both shaders and compiles program
fn load_program_with_tess<F: glium::backend::Facade>(display: &F, 
                                                     vs_name: &str, 
                                                     tc_name: &str,
                                                     te_name: &str,
                                                     gs_name: &str,
                                                     fs_name: &str) -> Option<glium::Program>
{
    // load vs
    let vs = read_shader(vs_name);
    if vs.is_err(){
        println!("could not read {}", vs_name);
        return None;
    }
    // load tc
    let tc = read_shader(tc_name);
    if tc.is_err(){
        println!("could not read {}", tc_name);
        return None;
    }

    // load te
    let te = read_shader(te_name);
    if te.is_err(){
        println!("could not read {}", te_name);
        return None;
    }
    // load gs
    let gs = read_shader(gs_name);
    if gs.is_err(){
        println!("could not read {}", gs_name);
        return None;
    }

    // load fs
    let fs = read_shader(fs_name);
    if fs.is_err(){
        println!("could not read {}", fs_name);
        return None;
    }

	let gs_str = gs.unwrap();
	let te_str = te.unwrap();
	let tc_str = tc.unwrap();

    let code = glium::program::SourceCode {
            vertex_shader: &vs.unwrap(),
            fragment_shader: &fs.unwrap(),
            geometry_shader: Some(&gs_str),
            tessellation_control_shader: Some(&tc_str),
            tessellation_evaluation_shader: Some(&te_str),
        };

    // compile
    let prog = glium::Program::new(display, code);
        //compile_program(display, &vs.unwrap(), &fs.unwrap());
    if let Err(x) = prog{
        print_err(x); 
        return None; 
    }
    Some(prog.unwrap())
}

fn get_date(name: &str) -> time::SystemTime{
    let file = get_path_to_shader(name).unwrap();
    let metadata = fs::metadata(file).unwrap();
    metadata.modified().unwrap()
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Program wrapper:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug)]
pub enum ShaderError{
    CompileError,
}

/// listens to filesystem to reload if file was changed
pub struct ProgramReloader{
    program: glium::program::Program,
    paths: Vec<String>,
    date: time::SystemTime,
    last_check: f64,
}

impl ProgramReloader{

    //type Self = ProgramReloader;

    pub fn new<F: glium::backend::Facade>(display: &F, vs_name: &str, fs_name: &str) 
        -> Result<ProgramReloader, ShaderError>
    {
        println!("load shader from: {}.glsl {}.glsl", vs_name, fs_name);

        let prog = load_program(display, vs_name, fs_name);
        if prog.is_none(){ return Err(ShaderError::CompileError);}

        Ok(ProgramReloader{
            program: prog.unwrap(),
            paths: vec!(vs_name.to_string(), fs_name.to_string()),
            date: time::SystemTime::now(),
            last_check: 0.0,
        })
    }

    pub fn new_tes<F: glium::backend::Facade>(display: &F, 
                                              vs_name: &str, 
                                              tc_name: &str,
                                              te_name: &str,
                                              gs_name: &str,
                                              fs_name: &str) 
        -> Result<ProgramReloader, ShaderError>
    {
        println!("load shader from: {}.vs.glsl {}.fs.glsl", vs_name, fs_name);

        let prog = load_program_with_tess(display, vs_name, tc_name, te_name, gs_name, fs_name);
        if prog.is_none(){ return Err(ShaderError::CompileError);}

        Ok(ProgramReloader{
            program: prog.unwrap(),
            paths: vec!(vs_name.to_string(),
                        tc_name.to_string(),
                        te_name.to_string(),
                        gs_name.to_string(),
                        fs_name.to_string()),
            date: time::SystemTime::now(),
            last_check: 0.0,
        })
    }

    pub fn update<F: glium::backend::Facade>(&mut self, display: &F, delta: f64){
        
        self.last_check += delta;
        // one second?
        if self.last_check < 1.0 {
            return;
        }
        self.last_check = 0.0;

        for path in &self.paths{
            let date = get_date(path);
            if self.date < date{
                self.date = date;

                if self.paths.len() == 2{
                    if let Some(prog) = load_program(display,  &*self.paths[0], &*self.paths[1]){
                        println!(" ~~ shader update ~~ ");
                        self.program = prog;
                    }
                }
                else{
                    if let Some(prog) = load_program_with_tess(display, &*self.paths[0], 
                                                                        &*self.paths[1],
                                                                        &*self.paths[2],
                                                                        &*self.paths[3],
                                                                        &*self.paths[4]){
                        println!(" ~~ shader update ~~ ");
                        self.program = prog;
                    }
                }
            }
        }
    }
}

// make my program to undestand this type
use renderer::context;
impl context::Program for ProgramReloader{
    fn get_program(&self) -> &glium::program::Program{
        &self.program
    }
    fn with_tess(&self) -> bool{
        self.program.has_tessellation_shaders()
    }
}


#[cfg(test)]
mod tests {

    use super::ProgramReloader;
    use context::Program;

    use glium::glutin::HeadlessRendererBuilder;
    use glium::DisplayBuild;

    #[test]
    fn create() {
        if let Ok(ctx) = HeadlessRendererBuilder::new(100,100).build_glium(){

            let bad = ProgramReloader::new(&ctx, "nonsense", "geom.fs");
            assert!(bad.is_err());
            let good = ProgramReloader::new(&ctx, "test.vs", "test.fs");
            assert!(good.is_ok());
        }
    }

// deactivate, headless render only gives me gl 1.20, no tessellation
//    #[test]
//    fn create_tes() {
//        use glium::glutin::GlRequest as GLReq;
//        let glVer = GLReq::Latest;
//        if let Ok(ctx) = HeadlessRendererBuilder::new(100,100)
//        .with_gl(glVer)
//        .build_glium(){
//            let good = ProgramReloader::new_tes(&ctx, 
//                                                "terrain.vs",
//                                                "terrain.tc",
//                                                "terrain.te",
//                                                "terrain.gs",
//                                                "terrain.vs");
//            assert!(good.is_ok());
//            let prg = good.unwrap();
//            assert!(prg.with_tess());
//        }
//    }
}
