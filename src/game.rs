use crate::player::player_plugin;
use crate::weapon::weapon_plugin;
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_plugins((player_plugin, weapon_plugin));
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
