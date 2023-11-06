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
                commands.spawn((
                    // give it a marker
                    TileType::Grass,
                    // give it a 2D sprite to render on-screen
                    // (Bevy's SpriteBundle lets us add everything necessary)
                    SpriteBundle {
                        texture: asset_server.load("grass.png"),
                        transform: Transform::from_xyz((i as f32) * 32., (j as f32) * 32., -1.0),
                        // use the default values for all other components in the bundle
                        ..Default::default()
                    },
                ));
            } else {
                commands.spawn((
                    // give it a marker
                    TileType::Water,
                    // give it a 2D sprite to render on-screen
                    // (Bevy's SpriteBundle lets us add everything necessary)
                    SpriteBundle {
                        texture: asset_server.load("water.png"),
                        transform: Transform::from_xyz((i as f32) * 32., (j as f32) * 32., -1.0),
                        // use the default values for all other components in the bundle
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

pub fn update_tiles(commands: Commands) {}
