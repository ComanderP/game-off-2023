use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub active: bool,
}

impl Collider {
    pub fn is_colliding(
        &self,
        self_translate: Vec3,
        other_translate: Vec3,
        other_size: Vec2,
    ) -> bool {
        return collide(self_translate, self.size, other_translate, other_size).is_some();
    }
}
