use bevy::prelude::*;

use super::{collider::Collider, player::Player};

#[derive(Component)]
pub struct Unit {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Unit {
    pub fn move_and_slide(
        &self,
        transform: &mut Transform,
        direction: Vec2,
        speed: &Speed,
        colliders: &Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
        dtime: f32,
    ) {
        let speed = speed.0 * dtime;
        // Try to move separatelly on x and y axis to allow sliding near walls.
        let mut next_translation_x = transform.translation + (speed * direction).extend(0.);
        next_translation_x.y = transform.translation.y;
        let mut next_translation_y = transform.translation + (speed * direction).extend(0.);
        next_translation_y.x = transform.translation.x;
        let mut is_colliding = false;
        let mut will_collide_x = false;
        let mut will_collide_y = false;
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
                next_translation_y,
                self.size,
            ) {
                will_collide_y = true;
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
        if is_colliding || !will_collide_y {
            next_translation.y = next_translation_y.y;
        }

        transform.translation = next_translation;
    }
}
