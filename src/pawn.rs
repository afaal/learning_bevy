use bevy::{prelude::*, scene::scene_spawner_system}; 

pub struct Pawn;
pub struct Name(String); 

pub struct Age(u8); 

pub struct Position {
    pub x: f32, 
    pub y: f32,
    pub z: f32
}
   
pub fn create_pawn(mut commands: Commands,  mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, name: String, position: Position) {
    // commands.spawn_scene(asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0")); 
    
    commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load("models/pawn.gltf#Mesh0/Primitive0"),
        material: materials.add(Color::rgb(1.0, 0.7, 0.6).into()),
        transform: Transform::from_xyz(position.x, position.y, position.z),
        ..Default::default()
        })
        .insert(Pawn)
        .insert(Name(name))
        .insert(position)
        .insert(Age(20));
        
}

pub fn pawn_move_system(mut query: Query<&mut Transform, With<Pawn>>) {
    for mut transform in query.iter_mut() {
        // println!("hello {}!", name.0)
        transform.translation.x += 1.0; 
    }
}