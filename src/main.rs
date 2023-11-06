use bevy::prelude::*;
mod components;

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite, move_bun))
        .run();
}

#[derive(Component)]
struct AnimationIndices
{
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{
    let texture_handle = asset_server.load("bun.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
    // Spawn a sprite with the default texture atlas sprite
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        // Animation data
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        // Very bad way of moving the sprite lmfao
        Direction::None,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
)
{
    for (indices, mut timer, mut sprite) in &mut query
    {
        // SPRITESHEET GOES BRRRRR
        timer.tick(time.delta());
        if timer.just_finished()
        {
            sprite.index = if sprite.index == indices.last
            {
                indices.first
            }
            else
            {
                sprite.index + 1
            };
        }
    }
}

#[derive(Component)]
enum Direction
{
    None,
    Up,
    Down,
    Left,
    Right,
}

// Very bad
fn move_bun(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Direction, &mut Transform)>)
{
    for (mut direction, mut transform) in &mut query
    {
        *direction = Direction::None;
        if keyboard_input.pressed(KeyCode::Up)
        {
            *direction = Direction::Up;
        }
        if keyboard_input.pressed(KeyCode::Down)
        {
            *direction = Direction::Down;
        }
        if keyboard_input.pressed(KeyCode::Left)
        {
            *direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right)
        {
            *direction = Direction::Right;
        }
        match *direction
        {
            Direction::Up => transform.translation.y += 1.0,
            Direction::Down => transform.translation.y -= 1.0,
            Direction::Left => transform.translation.x -= 1.0,
            Direction::Right => transform.translation.x += 1.0,
            _ => (),
        }
    }
}
