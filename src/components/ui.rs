use super::player::*;
use super::unit::*;
use bevy::prelude::*;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
enum PlayerStat {
    Health,
    Experience,
    Speed,
}

fn load_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                PlayerStat::Experience,
                TextBundle {
                    text: Text::from_section(
                        format!("Experience: {} {}", 0, 1000),
                        TextStyle::default(),
                    ),
                    ..default()
                },
            ));
            parent.spawn((
                PlayerStat::Health,
                TextBundle {
                    text: Text::from_section(
                        format!("Health: {}/{}", 100, 125),
                        TextStyle::default(),
                    ),
                    ..default()
                },
            ));
            parent.spawn((
                PlayerStat::Speed,
                TextBundle {
                    text: Text::from_section(
                        format!("Health: {}/{}", 100, 125),
                        TextStyle::default(),
                    ),
                    ..default()
                },
            ));
        });
}

fn update_ui(
    query: Query<(&Health, &Xp, &Speed), With<Player>>,
    mut stats: Query<(&mut Text, &PlayerStat)>,
) {
    let (health, xp, speed) = query.single();
    for (mut text, stat) in stats.iter_mut() {
        match stat {
            PlayerStat::Experience => {
                *text = Text::from_section(format!("Exp: {}/{}", xp.0, 1000), TextStyle::default())
            }
            PlayerStat::Speed => {
                *text = Text::from_section(format!("Speed: {}", speed.0), TextStyle::default())
            }
            PlayerStat::Health => {
                *text = Text::from_section(
                    format!("Health: {}/{}", health.current, health.max),
                    TextStyle::default(),
                )
            }
        }
    }
}
