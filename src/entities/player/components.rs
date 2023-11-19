use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Slash(pub Timer);

#[derive(Component)]
pub struct FishingFloat(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct StateTimer(pub Timer);

#[derive(Component)]
pub enum AnimationState
{
    Idle,
    Moving,
    Attacking,
    FishingCharging,
    Fishing,
}

#[derive(Component)]
pub struct Xp(pub u32);
