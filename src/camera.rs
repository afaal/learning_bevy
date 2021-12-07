use bevy::prelude::*; 
use bevy::input::mouse::{MouseWheel};

pub struct Camera; 

struct CameraView {
    radius: f32,
    focus: Vec3
}

impl Plugin for Camera {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init.system())
           .add_system(camera_move.system());
    }
}

fn camera_move(
    keys: Res<Input<KeyCode>>,
    mut ev_scroll: EventReader<MouseWheel>,
    query: Query<(&mut Transform, &mut CameraView), With<CameraView>>
) {
    let mut change_x = 0.0; 
    let mut change_z = 0.0;
    let mut scroll  = 0.0;
    let mut modified = false; 

    if keys.pressed(KeyCode::A) {
        modified = true;
        change_x -= 0.5;
    }
    if keys.pressed(KeyCode::D) {
        modified = true;
        change_x += 0.5;
    }
    if keys.pressed(KeyCode::W) {
        modified = true;
        change_z -= 0.5;
    }
    if keys.pressed(KeyCode::S) {
        modified = true;
        change_z += 0.5;
    }

    for ev in ev_scroll.iter() {
        modified = true;
        scroll += ev.y;
    }  

    if modified {
        query.for_each_mut(|(mut camera, mut camera_view)| {
            
            if change_x != 0.0 || change_z != 0.0 {
                camera.translation.x += change_x;
                camera.translation.z += change_z;
                camera_view.focus.x += change_x; 
                camera_view.focus.z += change_z; 

            } else if scroll.abs() > 0.0 {
                // modified = true;
                // dont allow zoom to reach zero or you get stuck
                camera_view.radius -= scroll * camera_view.radius * 0.05;
                camera_view.radius = f32::max(camera_view.radius, 2.5);

                let rot_matrix = Mat3::from_quat(camera.rotation);
                camera.translation = camera_view.focus + rot_matrix.mul_vec3(Vec3::new(1.0,1.0, camera_view.radius));
            }

        }) 
    }    
}

fn init(
    mut commands: Commands
) {
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 15.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(CameraView {
        radius: Vec3::new(0.0, 15.0, -1.0).length(),
        focus: Vec3::ZERO
    });
}