use super::collider::*;
use bevy::prelude::*;
use rand::Rng;

pub struct TilePlugin;

#[derive(Component)]
pub enum TileType {
    Grass,
    Water,
}

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tiles)
            .add_systems(Update, update_tiles);
    }
}

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in -10..=10 {
        for j in -10..=10 {
            let mut rng = rand::thread_rng();

            if rng.gen::<bool>() {
                let mut path = "grass_var1.png";
                if rng.gen::<i32>() % 20 == 0 {
                    path = "grass_var2.png";
                }
                commands.spawn((
                    TileType::Grass,
                    SpriteBundle {
                        texture: asset_server.load(path),
                        transform: Transform::from_xyz((i as f32) * 32., (j as f32) * 32., -1.0),
                        ..Default::default()
                    },
                ));
            } else {
                commands.spawn((
                    TileType::Water,
                    Collider {
                        size: Vec2::new(32., 32.),
                        active: true,
                    },
                    SpriteBundle {
                        texture: asset_server.load("water.png"),
                        transform: Transform::from_xyz((i as f32) * 32., (j as f32) * 32., -1.0),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub fn update_tiles(commands: Commands) {}
