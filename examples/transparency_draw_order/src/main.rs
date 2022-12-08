use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Transparency Draw Order".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(1.4933096, 4.8070683, -9.277165), // position
        vec3(0.14315122, 2.369473, -3.7785282), // target
        vec3(0.0, 1.0, 0.0), // up
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    let mut thin_cube = CpuMesh::cube();
    thin_cube.transform(&Mat4::from_nonuniform_scale(1.0, 1.0, 0.1));

    // Instanced mesh object, initialise with empty instances.
    let v = three_d::renderer::geometry::Instances {
        transformations: vec![
            Mat4::from_translation(vec3(0.0, 0.0, -2.0)),
            Mat4::from_translation(vec3(0.0, 0.0, -1.0)),
            Mat4::from_translation(vec3(0.0, 0.0, 0.0)),
            Mat4::from_translation(vec3(0.0, 0.0, 1.0)),
        ],
        colors: Some( vec![
                Color::new(0, 255, 0, 255),   // green, closest, should obscure everything.
                Color::new(255, 0, 255, 255), // purple, behind green, second opaque plane.
                Color::new(255, 0, 0, 128),   // Red, third plane, should be behind two opaques, blend in front of blue.
                Color::new(0, 0, 255, 128), // Furthest, blue layer.
            ]
        ),
        ..Default::default()
    };
    let mut transparent_meshes = Gm::new(
        InstancedMesh::new(&context, &v, &thin_cube),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );

    let mut thin_cube_right = CpuMesh::cube();
    thin_cube_right.transform(&(Mat4::from_translation(vec3(-4.0, 0.0, 0.0)) * Mat4::from_nonuniform_scale(1.0, 1.0, 0.1)));


    let mut opaque_meshes = Gm::new(
        InstancedMesh::new(&context, &v, &thin_cube_right),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );


    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        println!("camera.target: {:?}", camera.target());
        println!("camera.position: {:?}", camera.position());

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                transparent_meshes
                    .into_iter()
                    .chain(&opaque_meshes),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
}
