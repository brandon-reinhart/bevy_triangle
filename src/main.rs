use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut triangle = Mesh::new(PrimitiveTopology::TriangleList);
    triangle.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, -0.5, 0.0], [-1.0, -0.5, 0.0], [0.0, 0.5, 0.0]],
    );
    triangle.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0]],
    );
    triangle.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
    );
    triangle.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            Color::RED.as_rgba_u32(),
            Color::GREEN.as_rgba_u32(),
            Color::BLUE.as_rgba_u32(),
        ],
    );
    triangle.set_indices(Some(Indices::U32(vec![0, 1, 2])));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(triangle),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            unlit: true,
            ..default()
        }),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy_triangle".to_string(),
            width: 1024.0,
            height: 768.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgba(0.1, 0.0, 0.25, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_triangle)
        .run();
}
