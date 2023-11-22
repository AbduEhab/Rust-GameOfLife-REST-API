//! Demonstrates the simplest usage

use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, utils::petgraph::visit::EdgeRef};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use rand::prelude::*;

use minreq;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn 12*12*12 cubes with some padding between them
    for x in 0..12 {
        for y in 0..12 {
            for z in 0..12 {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * 1.2 - 6.0,
                            y as f32 * 1.2 - 6.0,
                            z as f32 * 1.2 - 6.0,
                        )),
                        ..Default::default()
                    })
                    .insert(id(x * 12 * 12 + y * 12 + z));
            }
        }
    }

    let mut cells_res = Cells {
        state: [false; 12 * 12 * 12],
    };

    for i in 0..12 * 12 * 12 {
        cells_res.state[i] = rand::thread_rng().gen_bool(0.5);
    }

    commands.insert_resource(cells_res);

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
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0., 50.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

#[derive(Component)]
struct id(usize);

#[derive(Resource)]
struct Cells {
    state: [bool; 12 * 12 * 12],
}

fn process_cells(
    cells: ResMut<Cells>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_data: Query<(&id, &Handle<StandardMaterial>)>,
) {
    
    let unchanged_cells = cells.state.clone();

    for (cell_id, material_handle) in material_data.iter_mut() {
        // find the cell's neighbors (8)
        let mut neighbors = 0;

        let cell_id = cell_id.0;

        let (x, y, z) = (cell_id / (12 * 12), (cell_id / 12) % 12, cell_id % 12);

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

                    let neighbor_id = x * 12 * 12 + y * 12 + z;

                    if neighbor_id >= 12 * 12 * 12 {
                        continue;
                    }

                    if unchanged_cells[neighbor_id as usize] {
                        neighbors += 1;
                    }
                }
            }
        }

        // get the new state of the cell
        let new_state = minreq::get(format!(
            "http://127.0.0.1:3000/api/gof?neigbors={}&state={}",
            neighbors.to_string(), unchanged_cells[cell_id].to_string()
        ))
        .send();

        if let Err(err) = new_state {
            print!("{}", err);
        }

    return;

        // let material = materials.get_mut(material_handle).unwrap();

        // // update the cell's material
        // if new_state.as_str().unwrap() == "true" {
        //     material.base_color = Color::rgb(1.0, 0.0, 0.0);
        // } else {
        //     material.base_color = Color::rgb(0.0, 0.0, 0.0);
        // }
    }

}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, process_cells)
        .run();
}
