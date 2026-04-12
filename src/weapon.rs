use crate::{common_components::Speed, player::Player};
use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollidingEntities, GravityScale, RigidBody, Sensor, Velocity,
};
use rand::{RngExt, rng};
use std::{ops::Range, time::Duration};

pub fn weapon_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            debug_give_player_weapon,
            tick_timers,
            fire_weapons,
            update_bullets,
        ),
    );
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub direction: Vec2,
}

#[derive(Component)]
struct WithDuration(Timer);

impl WithDuration {
    fn new(duration: f32) -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(duration),
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component, Debug)]
struct Weapon {
    pub damage: f32,
}

impl Weapon {
    fn fire(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        origin: Vec3,
    ) {
        commands.spawn(bullet(
            Vec2::new(
                rng().random_range::<f32, Range<f32>>(-1.0..1.),
                rng().random_range::<f32, Range<f32>>(-1.0..1.),
            ),
            3.,
            self.damage,
            100.,
            meshes,
            materials,
            origin,
        ));
    }
}

fn debug_give_player_weapon(
    mut commands: Commands,
    player: Single<Entity, (With<Player>, Without<Weapon>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyG) {
        commands
            .entity(player.into_inner())
            .insert((Weapon { damage: 5. }, WithDuration::new(1.)));
    }
}

fn tick_timers(mut timers: Query<&mut WithDuration>, time: Res<Time>) {
    for mut timer in timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn fire_weapons(
    mut commands: Commands,
    weapons: Query<(&Weapon, &WithDuration, Entity), With<Player>>,
    players: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (weapon, timer, entity) in weapons.iter() {
        if timer.0.is_finished() {
            let origin = players.get(entity).unwrap().translation;

            weapon.fire(&mut commands, &mut meshes, &mut materials, origin);
        }
    }
}

fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Speed, &mut Transform, &Bullet, &WithDuration)>,
) {
    for (entity, speed, mut transform, bullet, duration) in bullets.iter_mut() {
        if duration.0.is_finished() {
            commands.entity(entity).despawn();
        }

        // transform.translation += bullet.direction.extend(0.) * speed.0;
    }
}

fn bullet(
    direction: Vec2,
    range: f32,
    damage: f32,
    speed: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: Vec3,
) -> impl Bundle {
    (
        RigidBody::Dynamic,
        GravityScale(0.),
        Velocity {
            linvel: direction.normalize() * speed,
            angvel: 0.,
        },
        Sensor,
        Bullet { direction, damage },
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        Speed(speed),
        WithDuration::new(range),
        Collider::ball(2.),
        Mesh2d(meshes.add(Circle::new(2.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: origin,
            ..default()
        },
    )
}
