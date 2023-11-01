use bevy::prelude::*;
use super::{SceneEditor, SelectedEntity};
use bevy_rapier3d::prelude::*;

use crate::first_person_camera;

impl Plugin for SceneEditor{
    fn build(&self, app: &mut App){
        app.add_plugins((
            first_person_camera::FirstPersonCamera,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default()
        ))
        .add_systems(Startup, load_scene);
    }
}


fn load_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
    ){
        commands.spawn((PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
            ..default()
        },
        Collider::cuboid(2.5,0.0,2.5),
        ));
    
        commands.spawn((
            RigidBody::Dynamic{},
            Collider::cuboid(0.5,0.5,0.5),
            PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
            },
            
        ));
    
        commands.spawn(PointLightBundle{
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled:true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        });
    
        commands.spawn(
            TextBundle::from_section(
                "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
                Press 'P' to toggle perspective for line gizmos\n\
                Hold 'Left' or 'Right' to change the line width", 
                TextStyle{
                    font_size: 20.,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
        );
    }
