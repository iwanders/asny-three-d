use crate::core::*;
use crate::scene::*;
use crate::effects::*;

pub struct FogEffect {
    pub color: Vec3,
    pub density: f32,
    pub animation: f32,
    image_effect: ImageEffect
}

impl FogEffect {

    pub fn new(gl: &Gl) -> Result<FogEffect, Error>
    {
        Ok(FogEffect {color: vec3(0.8, 0.8, 0.8), density: 0.2, animation: 0.1, image_effect: ImageEffect::new(gl, include_str!("shaders/fog.frag"))?})
    }

    pub fn apply(&self, viewport: Viewport, camera: &camera::Camera, depth_texture: &Texture2D, time: f32) -> Result<(), Error>
    {
        let render_states = RenderStates {cull: CullType::Back, depth_mask: false,
            blend: Some(BlendParameters::new(BlendEquationType::Add,
                                             BlendMultiplierType::SrcAlpha,
                                             BlendMultiplierType::OneMinusSrcAlpha)),
            ..Default::default()};

        self.image_effect.program().use_texture(depth_texture, "depthMap")?;
        self.image_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
        self.image_effect.program().add_uniform_vec3("fogColor", &self.color)?;
        self.image_effect.program().add_uniform_float("fogDensity", &self.density)?;
        self.image_effect.program().add_uniform_float("animation", &self.animation)?;
        self.image_effect.program().add_uniform_float("time", &(0.001 * time))?;
        self.image_effect.program().add_uniform_vec3("eyePosition", camera.position())?;

        self.image_effect.apply(render_states, viewport)?;
        Ok(())
    }

}