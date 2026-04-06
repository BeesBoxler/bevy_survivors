use crate::{common_components::*, game::MainCamera, weapon::Bullet};
use bevy::{camera::Viewport, prelude::*};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollidingEntities, RigidBody, Sensor};
use rand::{RngExt, rng};
use std::time::Duration;

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Startup, create_enemy_spawner);
    app.add_systems(Update, (tick, bullet_hit, despawn_dead));
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemySpawner(pub Timer);

fn enemy_bundle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        Enemy,
        RigidBody::KinematicVelocityBased,
        Sensor,
        Health(5.),
        Speed::default(),
        Strength::default(),
        Collider::ball(15.),
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        Mesh2d(meshes.add(Circle::new(15.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::Srgba(
            Srgba::hex("#ff9900").unwrap(),
        )))),
    )
}

fn create_enemy_spawner(mut commands: Commands) {
    commands.spawn(EnemySpawner(Timer::new(
        Duration::from_secs(1),
        TimerMode::Repeating,
    )));
}

fn tick(
    mut commands: Commands,
    mut timers: Query<&mut EnemySpawner>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera: Single<(&Transform, &Camera), With<MainCamera>>,
) {
    let (camera_transform, camera) = camera.into_inner();

    if let Some(camera_rect) = camera.physical_viewport_rect() {
        for mut timer in timers.iter_mut() {
            timer.0.tick(time.delta());

            if timer.0.is_finished() {
                let x = rng().random_range(camera_rect.min.x..camera_rect.max.x) as f32
                    - camera_rect.max.y as f32 / 2.;
                let y = rng().random_range(camera_rect.min.y..camera_rect.max.y) as f32
                    - camera_rect.max.y as f32 / 2.;
                commands.spawn((
                    Transform {
                        translation: camera_transform.translation + Vec3::new(x, y, 0.),
                        ..default()
                    },
                    enemy_bundle(&mut meshes, &mut materials),
                ));
            }
        }
    }
}

fn bullet_hit(
    mut enemies: Query<&mut Health, With<Enemy>>,
    bullets: Query<(&CollidingEntities, &Bullet), With<Bullet>>,
) {
    for (hits, bullet) in bullets.iter() {
        for hit in hits.iter() {
            if let Ok(mut health) = enemies.get_mut(hit) {
                health.0 -= bullet.damage;
                return;
            }
        }
    }
}

fn despawn_dead(mut commands: Commands, enemies: Query<(Entity, &Health), With<Enemy>>) {
    for (entity, health) in enemies.iter() {
        if health.0 < 0. {
            commands.entity(entity).despawn();
        }
    }
}
