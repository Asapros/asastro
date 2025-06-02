use bevy::diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::control::settings::SimulationSettings;
use crate::view::follow::FollowInfo;

#[derive(Component)]
struct DiagnosticsText;

#[derive(Component)]
struct HelpText;

const HELP_TEXT: &'static str = r"========== CONTROLS ==========
move around - drag with RIGHT MB
zoom in/out - SCROLL
reset view - Z
toggle scale - N
toggle pause - SPACE
slow down simulation - ,
speed up simulation - .
reverse time - ;
setting frame of reference -
digit keys (1: Sun, ..., 9: Neptune) <<<
or click LEFT MB on a body <<<

speeding up too much messes up the orbits !!!
SPS is self-stabilising, so low FPS can also break !!!
";

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        DiagnosticsText
    ));


    commands.spawn((
        Text::new(HELP_TEXT),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Right),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        HelpText,
        Visibility::Visible
    ));

}

const SOLAR_YEAR_DAYS: f32 = 365.2422;

fn time_to_string(years: f32) -> String {
    if years.abs() > 1.0 {
        return format!("{:.1} years", years);
    }
    let days = years * SOLAR_YEAR_DAYS;

    if days.abs() > SOLAR_YEAR_DAYS / 12.0 {
        return format!("{:.1} months", days / SOLAR_YEAR_DAYS * 12.0)
    }
    if days.abs() > 1.0 {
        return format!("{:.1} days", days)
    }
    let minutes = days * 24.0 * 60.0;
    if minutes.abs() > 60.0 {
        return format!("{:.1} hours", minutes / 60.0);
    }
    if minutes.abs() > 1.0 {
        return format!("{:.1} minutes", minutes);
    }
    return format!("{:.1} seconds", minutes * 60.0);
}

fn update_diagnostic_text(mut text: Query<&mut Text, With<DiagnosticsText>>, diagnostics: Res<DiagnosticsStore>, settings: Res<SimulationSettings>, follow_info: Res<FollowInfo>) {
    let mut text = text.single_mut().expect("Diagnostic text not found");
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average());
    let fps_text = match fps {
        Some(fps) => format!("FPS: {}", fps.round()),
        None => "FPS: -".to_string()
    };
    let spf = settings.dt;
    let spf_text = format!("SPF: {}", time_to_string(spf));

    let sps_text = match fps {
        None => "SPS: -".to_string(),
        Some(fps) => format!("SPS: {}", time_to_string(fps as f32 * spf))
    };
    let sps_text_with_pause = match settings.pause {
        true => format!("{} [PAUSED]", sps_text),
        false => sps_text
    };
    
    let following_text = match &follow_info.name {
        None => "Reference: -".to_string(),
        Some(name) => format!("Reference: {}", name)
    };

    let normalized_text = match &settings.normalized {
        true => "Scale: normalized",
        false => "Scale: true"
    }.to_string();

    text.0 = [sps_text_with_pause, following_text, normalized_text].join("\n");
}

fn update_help_text(mut text: Query<&mut Visibility, With<HelpText>>, settings: Res<SimulationSettings>) {
    let mut help = text.single_mut().expect("Help text not found");
    *help = match settings.pause {
        true => Visibility::Visible,
        false => Visibility::Hidden
    }
}

pub(super) struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text);
        app.add_systems(Update, update_help_text);
        app.add_systems(Update, update_diagnostic_text);
    }
}