use bevy::prelude::*; 
use bevy_flycam::PlayerPlugin;
use bevy::window::{WindowDescriptor, WindowMode};

mod camera;
mod world;
mod sun; 
mod pawn;
use pawn::*;  
 
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Learning Bevy".to_string(),
            // width: 500.,
            // height: 300.,
            vsync: true,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(camera::Camera)
        .add_plugin(world::World)
        .add_plugin(sun::Sun)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        
        .add_system(pawn_move_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {



    create_pawn(commands, materials, asset_server, 
        "Brian".to_string(),
        Position{ x: 10.0, y: 10.0, z: 0.0 }
    ); 

}

