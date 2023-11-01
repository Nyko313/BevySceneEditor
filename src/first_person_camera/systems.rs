use bevy::{prelude::*, input::mouse::{MouseWheel, MouseMotion, MouseScrollUnit}};

use super::{FirstPersonCamera, components::SceneEditorCamera};


impl Plugin for FirstPersonCamera{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, init)
        .add_systems(Update, camera_controls);
    }
}

fn init(
    mut commands: Commands,
){
    commands.spawn(
        (Camera3dBundle{
            transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()},
        SceneEditorCamera{}
    ));
}

fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mouse_motion_evr: EventReader<MouseMotion>,
    scroll_evr: EventReader<MouseWheel>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>
){
    let mut camera_transform = camera_query.single_mut();
    let forward = camera_transform.forward();
    let right = camera_transform.right();

    if mouse.pressed(MouseButton::Right){
        camera_movement(keyboard,&mut camera_transform, forward, right);
        camera_look(mouse_motion_evr, &mut camera_transform, right);
    }

    camera_zoom(scroll_evr, &mut camera_transform, forward);
}

fn camera_look(
    mut mouse_motion_evr: EventReader<MouseMotion>,
    camera_transform: &mut Transform,
    right: Vec3
){
    let sensitivity = 0.006;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    for ev in mouse_motion_evr.iter(){
        yaw -= ev.delta.x * sensitivity;
        pitch -= ev.delta.y * sensitivity;

        pitch = pitch.clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

    }
    
    let yaw_rotation = Quat::from_axis_angle(Vec3::Y, yaw);
    let pitch_rotation = Quat::from_axis_angle(right, pitch);
    camera_transform.rotation = yaw_rotation * pitch_rotation * camera_transform.rotation;
}

fn camera_movement(
    mut keyboard: Res<Input<KeyCode>>,
    camera_transform: &mut Transform,
    forward: Vec3,
    right: Vec3
){
    let speed = 0.1;

    if keyboard.pressed(KeyCode::W){
        camera_transform.translation += forward * speed;
    }else if keyboard.pressed(KeyCode::S){
        camera_transform.translation -= forward * speed;
    }

    if keyboard.pressed(KeyCode::D){
        camera_transform.translation += right * speed;
    }else if keyboard.pressed(KeyCode::A){
        camera_transform.translation -= right * speed;
    }
}

fn camera_zoom (
    mut scroll_evr: EventReader<MouseWheel>,
    camera_transform: &mut Transform,
    forward: Vec3
    
){
    let mut zoom = 0.0;
        
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    zoom = ev.y;
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    zoom = ev.y;
                }
            }
        
        }

        camera_transform.translation += forward * zoom;
}
