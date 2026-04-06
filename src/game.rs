use crate::enemy::enemy_plugin;
use crate::player::player_plugin;
use crate::weapon::weapon_plugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_plugins((
        player_plugin,
        weapon_plugin,
        enemy_plugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
    ));
    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}
