use crate::{common_components::*, game::MainCamera};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_input_context::<Player>();
    app.add_observer(player_movement);
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(InputAction)]
#[action_output(Vec2)]
struct PlayerMove;

pub fn player_entity(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        Player,
        RigidBody::KinematicPositionBased,
        Speed(1.5),
        Strength::default(),
        Health(100.),
        Transform::default(),
        Mesh2d(meshes.add(Circle::new(15.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
        actions!(
            Player[(
                Action::<PlayerMove>::new(),
                Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick(), Cardinal::dpad()))
            )]
        ),
    )
}

fn spawn_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(player_entity(meshes, materials));
}

fn player_movement(
    movement: On<Fire<PlayerMove>>,
    player: Single<(&mut Transform, &Speed), With<Player>>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let (mut transform, speed) = player.into_inner();

    transform.translation += movement.value.extend(0.) * speed.0;
    camera.translation += movement.value.extend(0.) * speed.0;
}
