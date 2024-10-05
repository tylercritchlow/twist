use bevy::prelude::*;
mod scramblegeneration;

fn main() {
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
}

fn spacebar_timer_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut timer_state: ResMut<TimerState>,
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
            text.sections[0].style.color = Color::srgb(0.0, 1.0, 0.0); // Green
        } else {
            text.sections[0].style.color = Color::srgb(1.0, 0.0, 0.0); // Red
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
