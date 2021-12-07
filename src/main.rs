use bevy::prelude::*; 
use bevy_flycam::PlayerPlugin;

mod camera;
mod world;
 
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(camera::Camera)
        .add_plugin(world::World)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-0.5, 0.5, -0.5),
        ..Default::default()
    });

    
    // light
    commands.spawn_bundle(LightBundle {
        light: Light {
            range: 400.0,
            intensity: 9000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 30.0, 5.0),
        ..Default::default()
    }); 
}

