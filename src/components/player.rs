use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use super::ui::*;

use super::tiles::Collider;
const PLAYER_SIZE: Vec2 = Vec2::new(16., 30.);
pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerSettings {
    camera_locked: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSettings {
            camera_locked: true,
        });
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_update, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Xp(pub u32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

pub fn spawn_player(
    // needed for creating/removing data in the ECS World
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn((
        // give it a marker
        Player,
        // give it health and xp
        Health {
            current: 100,
            max: 125,
        },
        Xp(0),
        Speed(100.),
        // give it a 2D sprite to render on-screen
        // (Bevy's SpriteBundle lets us add everything necessary)
        SpriteBundle {
            texture: asset_server.load("man_transp.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    ));
}

pub fn player_update(
    mut players: Query<(&mut Transform, &Player, &Speed), Without<Camera>>,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Player>, Without<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player, speed) in &mut players {
        let speed = speed.0 * time.delta_seconds();
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::W) {
            direction.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        if input.just_pressed(KeyCode::Y) {
            settings.camera_locked = !settings.camera_locked;
        }

        let direction = direction.normalize_or_zero();
        let mut next_translation_x = transform.translation + (speed * direction).extend(0.);
        next_translation_x.y = transform.translation.y;
        let mut next_translation_y = transform.translation + (speed * direction).extend(0.);
        next_translation_y.x = transform.translation.x;
        let mut is_colliding = false;
        let mut will_collide_x = false;
        let mut will_collide_y = false;
        for (collider_transform, collider) in colliders.iter() {
            if collide(next_translation_x, PLAYER_SIZE, collider_transform.translation,collider.size).is_some() {
                will_collide_x = true;
            }
            if collide(next_translation_y, PLAYER_SIZE, collider_transform.translation,collider.size).is_some() {
                will_collide_y = true;
            }
            if collide(transform.translation, PLAYER_SIZE, collider_transform.translation,collider.size).is_some() {
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

        if settings.camera_locked || input.pressed(KeyCode::Space) {
            for (_, mut camera_transform) in &mut camera {
                camera_transform.translation = transform.translation;
            }
        }
    }
}

fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health, &mut Speed)>,
) {
    for (mut xp, mut health, mut speed) in query.iter_mut() {
        if xp.0 >= 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
            speed.0 += 10.0;
        }
    }
}
