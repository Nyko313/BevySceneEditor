use bevy::prelude::*;

pub mod systems;
pub mod first_person_camera;

pub struct SceneEditor;

#[derive(Component, Default, Clone)]
pub struct SelectedEntity;