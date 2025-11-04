// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;
use three_d_asset::texture::TextureData;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Logo!".to_string(),
        max_size: Some((512, 512)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.2),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    );

    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/rust_logo.png"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/rust_logo.png",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };
    let image = Texture2D::new(&context, &loaded.deserialize("").unwrap());

    let positions = vec![
        vec3(0.55, -0.4, 0.0),  // bottom right
        vec3(-0.55, -0.4, 0.0), // bottom left
        vec3(0.0, 0.6, 0.0),    // top
    ];
    let colors = vec![
        Srgba::new(255, 0, 0, 255), // bottom right
        Srgba::new(0, 255, 0, 255), // bottom left
        Srgba::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let l = -0.6;
    let r = 0.6;
    let t = 0.6;
    let b = -0.6;
    let y = 0.05;
    let positions = vec![
        vec3(l, b + y, 0.0), // left bottom
        vec3(r, b + y, 0.0), // right bottom
        vec3(l, t + y, 0.0), // left top
        vec3(r, b + y, 0.),  // right bottom
        vec3(r, t + y, 0.),  // right top
        vec3(l, t + y, 0.),  // left top
    ];
    let colors = vec![
        Srgba::new(255, 0, 0, 0),     // left bottom
        Srgba::new(255, 128, 255, 0), // right bottom
        Srgba::new(0, 255, 0, 0),     // left top
        Srgba::new(255, 128, 255, 0), // right bottom
        Srgba::new(0, 0, 255, 0),     // right top
        Srgba::new(0, 255, 0, 0),     // left top
    ];
    let cpu_mesh2 = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),

        ..Default::default()
    };

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());
    let mut model2 = Gm::new(
        Mesh::new(&context, &cpu_mesh2),
        ColorMaterial::new_transparent(
            &context,
            &CpuMaterial {
                ..Default::default()
            },
        ),
    );
    model2.material.is_transparent = true;

    model2.material.render_states = RenderStates {
        write_mask: WriteMask {
            red: true,
            green: true,
            blue: true,
            alpha: false,
            depth: false,
        },
        blend: Blend::ADD,
        ..Default::default()
    };

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))
            .apply_screen_material(&LogoMaterial { image: &image }, &camera, &[])
            .render(&camera, model2.into_iter().chain(&model), &[]);

        let z = frame_input.screen().read_color::<[u8; 4]>();
        let t = TextureData::RgbaU8(z);
        let t = three_d_asset::material::Texture2D {
            data: t,
            name: "logo".to_owned(),
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
            min_filter: Default::default(),
            mag_filter: Default::default(),
            mipmap: Default::default(),
            wrap_s: three_d::Wrapping::ClampToEdge,
            wrap_t: three_d::Wrapping::ClampToEdge,
        };
        use three_d_asset::io::Serialize;
        let _ = t.serialize("/tmp/logo_transparent.png").unwrap().save();
        FrameOutput::default()
    });
}

struct LogoMaterial<'a> {
    image: &'a Texture2D,
}

impl Material for LogoMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shader.frag").to_string()
    }

    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(0b1u16)
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_texture("image", self.image);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::STANDARD_TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}
