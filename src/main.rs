use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use components::bun::*;
use components::enemy::EnemyPlugin;
use components::player::*;
use components::tiles::*;
use components::ui::*;
mod components;
use bevy_sprite3d::*;
use components::world::*;
use bevy_asset_loader::prelude::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState { #[default] Loading, Ready }

const CAMERA_OFFSET : Vec3 = Vec3::new(0., 10., 25.);

#[derive(AssetCollection, Resource, Default)]
struct MyAssets {
    #[asset(path = "man_transp.png")]
    player: Handle<Image>,
    
    #[asset(path = "rock.png")]
    rock: Handle<Image>,
    
    #[asset(path = "water.png")]
    water: Handle<Image>,
    #[asset(path = "grass_var1.png")]
    grass: Handle<Image>,
    
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        //.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Sprite3dPlugin)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Ready)
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading)
        //.add_plugins(ShapePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, game_setup)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        //.add_plugins(EnemyPlugin)
        .add_plugins(TilePlugin)
        //.add_plugins(BunPlugin)
        //.add_plugins(UIPlugin)
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::WHITE.into()),
    //     transform: Transform::from_xyz(0., 1., 0.),
    //     ..default()
    // });

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
        transform: Transform::from_translation(CAMERA_OFFSET).with_rotation(Quat::from_rotation_x(-0.4)),
        ..default()
    });
}
