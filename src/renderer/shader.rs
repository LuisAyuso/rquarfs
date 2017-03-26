use regex::*;
use glium;

use std::path;
use std::fs;
use std::vec::*;
use std::time;

fn print_err(err: glium::program::ProgramCreationError) {
    match err {
        glium::program::ProgramCreationError::CompilationError(x) => println!("{}", x),
        glium::program::ProgramCreationError::LinkingError(y) => println!("{}", y),
        z => println!("{:?}", z),
    }
}

/// convert Option<String> into Option<&str>
fn get_slice(a: &Option<String>) -> Option<&str> {
    match a.as_ref() {
        Some(x) => Some(x.as_str()),
        None => None,
    }
}

/// loads both shaders and compiles program
fn load_program<F: glium::backend::Facade>(display: &F,
                                           path: &path::PathBuf)
                                           -> Option<glium::Program> {
    let code = ShaderPack::new(path);
    if let Err(x) = code {
        println!("{:?}", x);
        return None;
    }
    let code = code.unwrap();

    let glium_code = glium::program::SourceCode {
        vertex_shader: code.vertex.as_str(),
        fragment_shader: code.fragment.as_str(),
        geometry_shader: get_slice(&code.geom),
        tessellation_control_shader: get_slice(&code.tess_control),
        tessellation_evaluation_shader: get_slice(&code.tess_eval),
    };

    // compile
    let prog = glium::Program::new(display, glium_code);
    // compile_program(display, &vs.unwrap(), &fs.unwrap());
    if let Err(x) = prog {
        print_err(x);
        return None;
    }

    let prog = prog.unwrap();
    println!("Shader program loaded:");
    println!("   has tessellation: {}", prog.has_tessellation_shaders());
    println!("   uses point size: {}", prog.uses_point_size());
    println!("   srgb output: {}", prog.has_srgb_output());
    println!("   uniforms: ");
    for (name, _) in prog.uniforms() {
        println!("     - {}", name);
    }
    println!("   uniform bloks: ");
    for name in prog.get_uniform_blocks().keys() {
        println!("     - {}", name);
    }
    println!("   shader storage bloks: ");
    for name in prog.get_shader_storage_blocks().keys() {
        println!("     - {}", name);
    }

    Some(prog)
}

fn get_date(name: &path::PathBuf) -> time::SystemTime {
    let metadata = fs::metadata(name).unwrap();
    metadata.modified().unwrap()
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Program wrapper:
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// listens to filesystem to reload if file was changed
pub struct ProgramReloader {
    program: glium::program::Program,
    path: path::PathBuf,
    date: time::SystemTime,
    last_check: f64,
}

impl ProgramReloader {
    pub fn new<F: glium::backend::Facade>(display: &F,
                                          name: &str)
                                          -> Result<ProgramReloader, ShaderParseError> {
        println!("load shader from: {}.glsl", name);

        let mut path = fs::canonicalize(".").unwrap();
        path.push("shaders");
        path.push(format!("{}.glsl", name));

        let prog = load_program(display, &path);
        if prog.is_none() {
            return Err(ShaderParseError::CompileError);
        }

        Ok(ProgramReloader {
            program: prog.unwrap(),
            path: path,
            date: time::SystemTime::now(),
            last_check: 0.0,
        })
    }

    pub fn update<F: glium::backend::Facade>(&mut self, display: &F, delta: f64) {

        self.last_check += delta;
        // one second?
        if self.last_check < 1.0 {
            return;
        }
        self.last_check = 0.0;

        let date = get_date(&self.path);
        if self.date < date {
            self.date = date;

            if let Some(prog) = load_program(display, &self.path) {
                println!(" ~~ shader updated ~~ ");
                self.program = prog;
            }
        }
    }
}

// make my program to undestand this type
use renderer::context;
impl context::Program for ProgramReloader {
    fn get_program(&self) -> &glium::program::Program {
        &self.program
    }
    fn with_tess(&self) -> bool {
        self.program.has_tessellation_shaders()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Shader Pack,
//  this implements One file shaders, one day this would be an stand alone library
//  we do not compile the shaders, just do a basic parsing to extract them from a source code-like
//  file
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug)]
pub enum ShaderParseError {
    InvalidPath,
    SyntaxError(String),
    MissingShader,
    CompileError,
}

#[derive(Debug)]
struct ShaderPack {
    vertex: String,
    fragment: String,
    tess_control: Option<String>,
    tess_eval: Option<String>,
    geom: Option<String>,
}

impl ShaderPack {
    fn new(path: &path::PathBuf) -> Result<ShaderPack, ShaderParseError> {

        // parse file
        let units = parse_file(path)?;

        let mut res = ShaderPack {
            vertex: "".to_string(),
            fragment: "".to_string(),
            tess_control: None,
            tess_eval: None,
            geom: None,
        };

        for (kind, code) in units {

            match kind {
                ParseState::Common => {} // not implemented yet
                ParseState::Vertex => res.vertex = code,
                ParseState::Fragment => res.fragment = code,
                ParseState::TessC => res.tess_control = Some(code),
                ParseState::TessE => res.tess_eval = Some(code),
                ParseState::Geom => res.geom = Some(code),
            }

        }

        // validate, fragment and vertex can not be empty
        if res.vertex.is_empty() || res.fragment.is_empty() {
            println!("{:?}", res);
            return Err(ShaderParseError::MissingShader);
        }

        Ok(res)
    }
}

// parse state machine
// we are looking for // <- TEXT
// very basic regex line based parsing
#[derive(Copy, Clone, Debug)]
enum ParseState {
    Common,
    Vertex,
    Fragment,
    TessC,
    TessE,
    Geom,
}

type ShaderParse = (ParseState, String);

lazy_static! {
    static ref RE: Regex = Regex::new(r"\s*//\s*<-\s*(\w+)").unwrap();
}

struct Parser {
    accum: String,
    state: ParseState,
    items: Vec<ShaderParse>,
}

impl Parser {
    fn new() -> Parser {
        use self::ParseState::*;
        Parser {
            accum: "".to_string(),
            state: Common,
            items: Vec::new(),
        }
    }

    fn parse_line(&mut self, line: String) -> Result<(), ShaderParseError> {
        if let Some(cap) = RE.captures(&line) {
            let name = cap.get(1).map_or("", |m| m.as_str()).to_lowercase();
            self.items.push((self.state, self.accum.clone()));
            self.accum = String::new();

            if name == "vertex" {
                self.state = ParseState::Vertex;
            } else if name == "fragment" {
                self.state = ParseState::Fragment;
            } else if name == "tessellation_control" {
                self.state = ParseState::TessC;
            } else if name == "tessellation_evaluation" {
                self.state = ParseState::TessE;
            } else if name == "geometry" {
                self.state = ParseState::Geom;
            } else {
                return Err(ShaderParseError::SyntaxError(format!("Unknow section {}", name)
                    .to_string()));
            }
        } else {
            self.accum.push_str(line.as_str());
            self.accum.push_str("\n");
        }
        Ok(())
    }
    fn get_items(mut self) -> Vec<ShaderParse> {
        self.items.push((self.state, self.accum));
        self.items
    }
}

fn parse_file(path: &path::PathBuf) -> Result<Vec<ShaderParse>, ShaderParseError> {

    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;
    use self::ShaderParseError::*;
    // use self::ParseState::*;

    let mut parser = Parser::new();

    if let Ok(f) = File::open(path) {
        let f = BufReader::new(f);

        for l in f.lines() {
            parser.parse_line(l.unwrap())?;
        }
        return Ok(parser.get_items());
    }

    Err(InvalidPath)
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Tests
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



#[cfg(test)]
mod tests {

    use super::ProgramReloader;
    use super::ShaderPack;

    use glium::glutin::HeadlessRendererBuilder;
    use glium::DisplayBuild;

    use std::fs;

    #[test]
    fn missing_file() {
        let mut path = fs::canonicalize(".").unwrap();
        path.push("shaders");
        path.push("error.glsl");

        let x = ShaderPack::new(&path);
        if let Ok(_) = x {
            assert!(false);
        }
    }

    #[test]
    fn single_file() {

        let mut path = fs::canonicalize(".").unwrap();
        path.push("shaders");
        path.push("geom.glsl");

        let x = ShaderPack::new(&path);
        match x {
            Err(_) => assert!(false),
            Ok(x) => println!("{:?}", x),
        }
    }

    #[test]
    fn create() {
        if let Ok(ctx) = HeadlessRendererBuilder::new(100, 100).build_glium() {

            let bad = ProgramReloader::new(&ctx, "nonsense");
            assert!(bad.is_err());
            let good = ProgramReloader::new(&ctx, "test");
            assert!(good.is_ok());
        }
    }
}
