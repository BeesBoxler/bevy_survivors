use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health(pub f32);

#[derive(Component, Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(1.)
    }
}

#[derive(Component, Debug)]
pub struct Strength(pub f32);

impl Default for Strength {
    fn default() -> Self {
        Strength(1.)
    }
}
