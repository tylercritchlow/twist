use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) mod scramblegeneration;

#[derive(Serialize, Deserialize)]
struct Session {
    id: u32,
}

#[derive(Resource, Serialize, Deserialize)]
struct SessionData {
    session: Session,
    scrambles: Vec<String>,
    times: Vec<f32>,
}
fn main() {
    println!("{}", dirs::config_dir().unwrap().join("twist").display());

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (spacebar_timer_system, update_timer_text))
        .run();
}

#[derive(Component)]
struct TimerText;

#[derive(Component)]
struct ScrambleText;

#[derive(Resource)]
struct TimerState {
    counting: bool,
    time_elapsed: f32,
    just_stopped: bool,
    held: bool,
    held_time: f32,
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            scramblegeneration::generate_scramble_string(20),
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            margin: UiRect::horizontal(Val::Auto),
            top: Val::Px(20.0),
            ..default()
        }),
        ScrambleText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0.00",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
        TimerText,
    ));

    commands.insert_resource(TimerState {
        counting: false,
        time_elapsed: 0.0,
        just_stopped: false,
        held: false,
        held_time: 0.0,
    });

    let config_dir = dirs::config_dir().unwrap().join("twist");
    commands.insert_resource(
        Persistent::<SessionData>::builder()
            .name("session data")
            .format(StorageFormat::Toml)
            .path(config_dir.join("sessiondata.toml"))
            .default(SessionData { session: Session { id: 0 }, scrambles: Vec::new(), times: Vec::new() })
            .build()
            .expect("failed to initialize session data")
    )}

fn spacebar_timer_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut timer_state: ResMut<TimerState>,
    session_data: ResMut<Persistent<SessionData>>,
    query: Query<&mut Text, With<ScrambleText>>,
) {
    let spacebar_pressed = keyboard_input.pressed(KeyCode::Space);
    let spacebar_released = keyboard_input.just_released(KeyCode::Space);

    if spacebar_pressed && !timer_state.held {
        timer_state.held = true;
    } else if spacebar_released {
        timer_state.held = false;
    }

    if spacebar_released && !timer_state.counting {
        if timer_state.just_stopped {
            timer_state.just_stopped = false;
        } else if timer_state.held_time >= 0.5 {
            timer_state.time_elapsed = 0.0;
            timer_state.held_time = 0.0;
            timer_state.counting = true;
        } else {
            timer_state.held_time = 0.0;
        }
    }

    if spacebar_pressed && !timer_state.counting {
        timer_state.held_time += time.delta_seconds();
    }else {
        timer_state.held_time = 0.0;
    }

    if spacebar_pressed && timer_state.counting {
        timer_state.counting = false;
        timer_state.just_stopped = true;
        timer_state.held_time = 0.0;

        update_session_data(session_data, timer_state.time_elapsed, query.iter().next().unwrap().sections[0].value.clone());
        update_scramble_text(query);
    }

    if timer_state.counting {
        timer_state.time_elapsed += time.delta_seconds();
    }
}

fn update_timer_text(
    mut query: Query<&mut Text, With<TimerText>>,
    timer_state: Res<TimerState>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("{:.2}", timer_state.time_elapsed);

        if timer_state.held_time >= 0.5 {
            text.sections[0].style.color = Color::srgb(0.0, 1.0, 0.0);
        } else {
            text.sections[0].style.color = Color::srgb(1.0, 0.0, 0.0);
        }

        if timer_state.counting || !timer_state.held {
            text.sections[0].style.color = Color::WHITE;
        }
    }
}

fn update_scramble_text(mut query: Query<&mut Text, With<ScrambleText>>) {
    for mut text in &mut query {
        text.sections[0].value = scramblegeneration::generate_scramble_string(20);
    }
}

fn update_session_data(mut session_data: ResMut<Persistent<SessionData>>, time: f32, scramble: String) {
    session_data.update(|session_data| {
        session_data.times.push(time);
    }).expect("failed to update session data");

    println!("{:?}", session_data.times);

    session_data.update(|session_data| {
        session_data.scrambles.push(scramble.clone());
    }).expect("failed to update session data");
}


#[cfg(test)]
mod tests {

    use crate::scramblegeneration::*;
    
    #[test]
    fn test_scramble_move_to_string() {
        assert_eq!(
            ScrambleMove {
                mv: Move::U,
                variation: MoveVariation::Normal
            }
            .to_string(),
            "U"
        );
        assert_eq!(
            ScrambleMove {
                mv: Move::R,
                variation: MoveVariation::Prime
            }
            .to_string(),
            "R'"
        );
        assert_eq!(
            ScrambleMove {
                mv: Move::F,
                variation: MoveVariation::Double
            }
            .to_string(),
            "F2"
        );
    }

    #[test]
    fn test_moves_cancel() {
        assert!(moves_cancel(
            &ScrambleMove {
                mv: Move::U,
                variation: MoveVariation::Normal
            },
            &ScrambleMove {
                mv: Move::U,
                variation: MoveVariation::Prime
            }
        ));
        assert!(moves_cancel(
            &ScrambleMove {
                mv: Move::R,
                variation: MoveVariation::Double
            },
            &ScrambleMove {
                mv: Move::R,
                variation: MoveVariation::Double
            }
        ));
        assert!(!moves_cancel(
            &ScrambleMove {
                mv: Move::L,
                variation: MoveVariation::Normal
            },
            &ScrambleMove {
                mv: Move::L,
                variation: MoveVariation::Double
            }
        ));
    }

    #[test]
    fn test_moves_repeat() {
        assert!(moves_repeat(
            &ScrambleMove {
                mv: Move::U,
                variation: MoveVariation::Normal
            },
            &ScrambleMove {
                mv: Move::U,
                variation: MoveVariation::Prime
            }
        ));
        assert!(!moves_repeat(
            &ScrambleMove {
                mv: Move::L,
                variation: MoveVariation::Normal
            },
            &ScrambleMove {
                mv: Move::R,
                variation: MoveVariation::Normal
            }
        ));
    }

    #[test]
    fn test_are_opposite_faces() {
        assert!(are_opposite_faces(&Move::U, &Move::D));
        assert!(are_opposite_faces(&Move::L, &Move::R));
        assert!(are_opposite_faces(&Move::F, &Move::B));
        assert!(!are_opposite_faces(&Move::U, &Move::L));
        assert!(!are_opposite_faces(&Move::F, &Move::R));
    }

    #[test]
    fn test_generate_scramble() {
        let scramble = generate_scramble(20);
        assert_eq!(scramble.len(), 20);

        for i in 1..scramble.len() {
            assert!(!moves_repeat(&scramble[i - 1], &scramble[i]));
            assert!(!moves_cancel(&scramble[i - 1], &scramble[i]));

            if i > 1 {
                assert!(!are_opposite_faces(&scramble[i - 2].mv, &scramble[i].mv)
                    || !are_opposite_faces(&scramble[i - 1].mv, &scramble[i].mv));
            }
        }
    }

    #[test]
    fn test_generate_scramble_string() {
        let scramble_str = generate_scramble_string(20);
        let moves: Vec<&str> = scramble_str.trim().split_whitespace().collect();
        assert_eq!(moves.len(), 20);
    }
}
