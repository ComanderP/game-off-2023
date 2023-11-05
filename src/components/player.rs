use bevy::prelude::*;

// Marker for the player
#[derive(Component)]
struct Player;

fn spawn_player(
    // needed for creating/removing data in the ECS World
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
)
{
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
        // give it a 2D sprite to render on-screen
        // (Bevy's SpriteBundle lets us add everything necessary)
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    ));
}

#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
struct Health
{
    current: u32,
    max: u32,
}

fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health)>,
)
{
    for (mut xp, mut health) in query.iter_mut()
    {
        if xp.0 > 1000
        {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
}
