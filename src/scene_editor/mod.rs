use bevy::prelude::*;

pub mod systems;

pub struct SceneEditor;

#[derive(Component, Default, Clone)]
pub struct SelectedEntity;