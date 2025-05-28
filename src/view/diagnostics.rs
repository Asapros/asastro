use bevy::diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::control::settings::SimulationSettings;
use crate::view::follow::FollowInfo;

#[derive(Component)]
struct DiagnosticsText;

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: 32.0,
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

fn update_text(mut text: Query<&mut Text, With<DiagnosticsText>>, diagnostics: Res<DiagnosticsStore>, settings: Res<SimulationSettings>, follow_info: Res<FollowInfo>) {
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
    // let sps_text_with_pause = match settings.pause {
    //     
    // }
    
    let following_text = match &follow_info.name {
        None => "Following: -".to_string(),
        Some(name) => format!("Following: {}", name)
    };

    text.0 = [sps_text, following_text].join("\n");
}

pub(super) struct DiagnosticsPlugin;
impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text);
        app.add_systems(Update, update_text);
    }
}