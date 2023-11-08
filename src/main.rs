use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use components::bun::*;
use components::enemy::EnemyPlugin;
use components::player::*;
use components::tiles::*;
use components::ui::*;
mod components;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ShapePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, game_setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
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
    /* Testing out the shapes library
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 1.0),
    ));
    */
}
