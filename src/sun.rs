use bevy::prelude::*; 


pub struct Sun; 


impl Plugin for Sun {
    fn build(&self, app: &mut AppBuilder) {

        app.add_startup_system(setup.system()); 
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // light
    commands.spawn_bundle(LightBundle {
        light: Light {
            range: 2000.0,
            fov: f32::to_radians(180.0),
            intensity: 40000.0,
            depth: 1000.0..20000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-0.0, 200.0, 0.0),
        ..Default::default()
    }); 
}
