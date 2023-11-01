use bevy::prelude::*;
use scene_editor::SceneEditor;

mod scene_editor;


// usare https://github.com/aevyrie/bevy_mod_picking
fn main() {
    App::new()
    .add_plugins((DefaultPlugins, SceneEditor))
    .run();
}
