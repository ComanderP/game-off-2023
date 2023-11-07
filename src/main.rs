use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use components::player::*;
use components::tiles::*;
use components::bun::*;
use components::ui::*;
mod components;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, game_setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(TilePlugin)
        .add_plugins(BunPlugin)
        .add_plugins(UIPlugin)
        .run();
}

fn game_setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 512.,
        min_height: 144.,
    };

    commands.spawn(camera);
}

