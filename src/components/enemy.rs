use super::collider::*;
use super::player::Player;
use super::unit::*;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(Update, (update_enemy, deal_damage));
    }
}
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MeleeRange(pub f32);

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("feesh_man_sheet.png");

    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 1, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Enemy,
        Health {
            current: 100,
            max: 125,
        },
        Speed(100.),
        Unit {
            size: Vec2::new(16., 30.),
        },
        MeleeRange(20.0),
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(200., 200., 0.),
            ..Default::default()
        },
    ));
}

pub fn update_enemy(
    mut enemies: Query<(&mut Transform, &Enemy, &Speed, &Unit), Without<Camera>>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    players: Query<(&mut Transform, &Player), (Without<Enemy>, Without<Camera>, Without<Collider>)>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    let player = players.single();
    for (mut transform, _, speed, unit) in &mut enemies {
        let direction = (player.0.translation - transform.translation).truncate();

        let direction = direction.normalize_or_zero();

        unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);
    }
}

fn deal_damage(
    mut enemies: Query<(&mut Transform, &Enemy, &MeleeRange)>,
    mut players: Query<(&mut Transform, &Player, &mut Health), Without<Enemy>>,
) {
    let mut player = players.single_mut();
    for (transform, _, range) in &mut enemies {
        if player.0.translation.distance(transform.translation) <= range.0 {
            if player.2.current > 10 {
                player.2.current -= 10;
            } else {
                player.2.current = 0;
            }
            // info!("Player hit")
        }
    }
}
