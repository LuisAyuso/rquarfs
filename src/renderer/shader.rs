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
    //print!("load shader: {:?}", path);
    Ok(path)
}


/// compiles text buffers into shader program
fn compile_program<F: glium::backend::Facade>(display: &F, vs: &str, fs: &str) 
        -> Result<glium::Program, glium::ProgramCreationError>{
   glium::Program::from_source(display, vs, fs, None)
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
    let prog = compile_program(display, &vs.unwrap(), &fs.unwrap());
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
    paths: [String; 2],
    date: time::SystemTime,
    last_check: f64,
}

impl ProgramReloader{

    //type Self = ProgramReloader;

    pub fn new<F: glium::backend::Facade>(display: &F, vs_name: &str, fs_name: &str) 
        -> Result<ProgramReloader, ShaderError>
    {
        println!("load shader from: {}.vs.glsl {}.fs.glsl", vs_name, fs_name);

        let prog = load_program(display, vs_name, fs_name);
        if prog.is_none(){ return Err(ShaderError::CompileError);}

        Ok(ProgramReloader{
            program: prog.unwrap(),
            paths: [vs_name.to_string(), fs_name.to_string()],
            date: time::SystemTime::now(),
            last_check: 0.0,
        })
    }

    pub fn update<F: glium::backend::Facade>(&mut self, display: &F, delta: f64){
        use std::cmp;

        self.last_check += delta;
        // one second?
        if self.last_check < 1.0 {
            return;
        }
        self.last_check = 0.0;

        let a = get_date(&*self.paths[0]);
        let b = get_date(&*self.paths[1]);

        // if any date is newer
        if self.date < a || self.date < b{
            self.date   = cmp::max(a,b);

            let prog = load_program(display,  &*self.paths[0], &*self.paths[1]);
            if prog.is_none() { return;}
            println!(" ~~ shader update ~~ ");

            self.program = prog.unwrap();
        }
    }
}

// make my program to undestand this type
use renderer::context;
impl context::Program for ProgramReloader{
    fn get_program(&self) -> &glium::program::Program{
        &self.program
    }
}




