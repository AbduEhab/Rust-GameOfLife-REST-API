//! Demonstrates the simplest usage

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use rand::prelude::*;

use minreq;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, process_cells)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn 32*32*32 cubes
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 - 16.0,
                            y as f32 - 16.0,
                            z as f32 - 16.0,
                        )),
                        ..Default::default()
                    })
                    .insert(Cell {
                        alive: rand::thread_rng().gen_bool(0.5),
                    })
                    .insert(id(x * 32 * 32 + y * 32 + z));
            }
        }
    }

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 3., 10.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

#[derive(Component)]
struct id(usize);

#[derive(Component)]
struct Cell {
    alive: bool,
}

fn process_cells(mut cells: Query<(&mut Cell, &id, &mut Handle<StandardMaterial>)>) {
    let i = 0;

    for (mut cell, cell_id, material) in cells.iter_mut() {
        // find the cell's neighbors (8)
        let mut neighbors = 0;

        let cell_id = cell_id.0;

        let (x, y, z) = (cell_id / (32 * 32), (cell_id / 32) % 32, cell_id % 32);

        // get the state of the 8 neighbors
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    let (x, y, z) = (
                        (x as i32 + i) as u32,
                        (y as i32 + j) as u32,
                        (z as i32 + k) as u32,
                    );

                    let neighbor_id = x * 32 * 32 + y * 32 + z;

                    if neighbor_id >= 32 * 32 * 32 {
                        continue;
                    }

                    let neighbor = cells.get_mut(id(neighbor_id as usize));

                    if neighbor.is_ok() {
                        if neighbor.unwrap().alive {
                            neighbors += 1;
                        }
                    }
                }
            }
        }
    }

    // let name = "world";

    // let res = minreq::get(format!("http://127.0.0.1:3000/api/hello?name={}", name))
    //     .send()
    //     .unwrap();

    // // form a string from the response's body
    // let body = res.as_str().unwrap();
    // println!("{}", body);
}

// get all
