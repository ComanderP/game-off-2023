use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, diagnostic::LogDiagnosticsPlugin};
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_sprite3d::*;

mod assets;
mod entities;
mod ui;
mod world;

use assets::MyAssets;
use entities::enemy::EnemyPlugin;
use entities::player::PlayerPlugin;
use entities::shop::ShopPlugin;
use entities::unit::Health;
use ui::UIPlugin;
use world::WorldPlugin;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default, Reflect)]
enum GameState
{
    #[default]
    Loading, // loading assets from files
    Spawning, // spawning the world
    Ready,    // game is running
}

fn main()
{
    App::new()
        .insert_resource(Msaa::Off)
        .register_type::<Health>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HealthBarPlugin::<Health>::default())
        // Show diagnostics in console
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(ShapePlugin) // plugin for drawing shapes on screen
        .add_plugins(Sprite3dPlugin)
        // define initial gamestate
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Spawning),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading)
        // the game world should be setup at OnEnter(GameState::Spawning)
        // to solve entities poping in at playtime
        .add_systems(
            Update,
            finish_spawning.run_if(in_state(GameState::Spawning)),
        )
        .add_systems(OnEnter(GameState::Ready), game_setup)
        // systems that rely on the player being spawned should: run_if(in_state(GameState::Ready))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        // handle spawning and updating game components
        .add_plugins(EnemyPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ShopPlugin)
        //.add_plugins(BunPlugin)
        .add_plugins(UIPlugin)
        .run();
}

fn finish_spawning(mut game_state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>)
{
    if input.pressed(KeyCode::Space)
    {
        game_state.set(GameState::Ready);
    }
}

fn game_setup(mut commands: Commands)
{
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.3, 1.0)),
            ..default()
        },
        projection: bevy::prelude::Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 6.0,
            ..default()
        }),
        transform: Transform::from_translation(entities::player::CAMERA_OFFSET)
            .with_rotation(Quat::from_rotation_x(-0.4)),
        ..default()
    });
}
