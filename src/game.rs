use crate::enemy::enemy_plugin;
use crate::player::player_plugin;
use crate::weapon::weapon_plugin;
use crate::xp::xp_plugin;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.add_plugins((
        player_plugin,
        weapon_plugin,
        enemy_plugin,
        xp_plugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
    ));

    app.insert_resource(LevelSelection::index(0));

    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            // scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk").into(),
        transform: Transform::from_xyz(-2048., -2048., -100.).with_scale(Vec3::splat(3.)),

        ..Default::default()
    },));
}
