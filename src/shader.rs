use std::path::Path;
use std::io;
use glium;
use std::fs::File;
use std::io::Read;

pub struct Shader;

impl Shader{
    pub fn load<F: glium::backend::Facade,
                P: AsRef<Path>>(display: &F,
                                vert_file: P,
                                frag_file: P) -> io::Result<glium::Program>{
        let mut frag_source = String::new();
        let mut vert_source = String::new();

        File::open(frag_file)?.read_to_string(&mut frag_source)?;
        File::open(vert_file)?.read_to_string(&mut vert_source)?;

        match glium::Program::from_source(display, vert_source.as_str(), frag_source.as_str(), None){
            Ok(p)  => Ok(p),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}
