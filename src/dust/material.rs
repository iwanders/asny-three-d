use dust::program;
use gl;
use dust::input;

#[derive(Debug)]
pub enum Error {
    Program(program::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

pub trait Shade {
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct Material {
    program: program::Program
}

impl Shade for Material {
    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), Error> {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }
}

impl Material
{
    pub fn create(gl: &gl::Gl) -> Result<Material, Error>
    {
        let shader_program = program::Program::from_resource(&gl, "assets/shaders/triangle")?;
        Ok(Material { program: shader_program })
    }

    pub fn get_attribute_location(&self, name: &str) -> Result<i32, Error> {
        let location = self.program.get_attribute_location(name)?;
        Ok(location)
    }

    pub fn apply(&self)
    {
        self.program.set_used();
    }
}
