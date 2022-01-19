use crate::core::*;
use crate::renderer::*;

///
/// A light which shines from the given position in all directions.
///
pub struct PointLight {
    pub intensity: f32,
    pub color: Color,
    pub position: Vec3,
    pub attenuation_constant: f32,
    pub attenuation_linear: f32,
    pub attenuation_exponential: f32,
}

impl PointLight {
    pub fn new(
        _context: &Context,
        intensity: f32,
        color: Color,
        position: &Vec3,
        attenuation_constant: f32,
        attenuation_linear: f32,
        attenuation_exponential: f32,
    ) -> ThreeDResult<PointLight> {
        Ok(PointLight {
            intensity,
            color,
            position: *position,
            attenuation_constant,
            attenuation_linear,
            attenuation_exponential,
        })
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32) {
        self.attenuation_constant = constant;
        self.attenuation_linear = linear;
        self.attenuation_exponential = exponential;
    }

    pub fn attenuation(&self) -> (f32, f32, f32) {
        (
            self.attenuation_constant,
            self.attenuation_linear,
            self.attenuation_exponential,
        )
    }

    pub fn set_position(&mut self, position: &Vec3) {
        self.position = *position;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }
}

impl Light for PointLight {
    fn shader_source(&self, i: u32) -> String {
        format!(
        "
            uniform vec3 color{};
            uniform vec3 attenuation{};
            uniform vec3 position{};

            vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
            {{
                vec3 light_direction = position{} - position;
                float distance = length(light_direction);
                light_direction = light_direction / distance;

                vec3 light_color = attenuate(color{}, attenuation{}, distance);
                return calculate_light(light_color, light_direction, surface_color, view_direction, normal, metallic, roughness);
            }}
        
        ", i, i, i, i, i, i, i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        program.use_uniform_vec3(
            &format!("color{}", i),
            &(self.color.to_vec3() * self.intensity),
        )?;
        program.use_uniform_vec3(
            &format!("attenuation{}", i),
            &vec3(
                self.attenuation_constant,
                self.attenuation_linear,
                self.attenuation_exponential,
            ),
        )?;
        program.use_uniform_vec3(&format!("position{}", i), &self.position)?;
        Ok(())
    }
    fn update_shadow(&mut self, _geometries: &[impl Geometry]) -> ThreeDResult<()> {
        Ok(())
    }
}
