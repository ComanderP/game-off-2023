use super::{collider::Collider, player::Player};
use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component)]
pub struct Unit {
    pub size: Vec2,
}

#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Reflect)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

impl Unit {
    pub fn move_and_slide(
        &self,
        transform: &mut Transform,
        direction: Vec3,
        speed: &Speed,
        colliders: &Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
        dtime: f32,
    ) {
        let speed = speed.0 * dtime;
        // Try to move separatelly on x and z axis to allow sliding near walls.
        let mut next_translation_x = transform.translation + (speed * direction);
        next_translation_x.z = transform.translation.z;
        let mut next_translation_z = transform.translation + (speed * direction);
        next_translation_z.x = transform.translation.x;
        let mut is_colliding = false;
        let mut will_collide_x = false;
        let mut will_collide_z = false;
        for (collider_transform, collider) in colliders.iter() {
            if collider.is_colliding(
                collider_transform.translation,
                next_translation_x,
                self.size,
            ) {
                will_collide_x = true;
            }
            if collider.is_colliding(
                collider_transform.translation,
                next_translation_z,
                self.size,
            ) {
                will_collide_z = true;
            }
            if collider.is_colliding(
                collider_transform.translation,
                transform.translation,
                self.size,
            ) {
                is_colliding = true;
            }
        }
        let mut next_translation = transform.translation;
        if is_colliding || !will_collide_x {
            next_translation.x = next_translation_x.x;
        }
        if is_colliding || !will_collide_z {
            next_translation.z = next_translation_z.z;
        }

        transform.translation = next_translation;
    }
}
