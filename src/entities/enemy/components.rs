use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MeleeRange(pub f32);

#[derive(Component)]
pub struct Cooldown(pub Timer);
