use bevy::prelude::Color;

// Using units:
// distance: AU
// time:     year
// mass:     MO (solar mass)
// This system normalizes gravitational constant G = 4π²

pub(super) struct TemplateBody {
    pub name: &'static str,
    pub mass: f32,          // [MO]
    pub radius: f32,        // [AU]
    pub aphelion_dist: f32, // [AU]
    pub aphelion_speed: f32,// [AU/year]
    pub color: Color
}

pub(super) const SOLAR_SYSTEM_TEMPLATE: [TemplateBody; 10] = [
    TemplateBody { name: "Sun",     mass: 1.0,         radius: 0.00465047,aphelion_dist: 0.0,   aphelion_speed: 0.0,  color: Color::srgb_u8(255, 223, 0) },
    TemplateBody { name: "Mercury", mass: 0.000000166, radius: 0.0000163, aphelion_dist: 0.468, aphelion_speed: 8.17, color: Color::srgb_u8(169, 169, 169) },
    TemplateBody { name: "Venus",   mass: 0.00000245,  radius: 0.0000405, aphelion_dist: 0.728, aphelion_speed: 7.38, color: Color::srgb_u8(218, 165, 32) },
    TemplateBody { name: "Earth",   mass: 0.00000300,  radius: 0.0000426, aphelion_dist: 1.017, aphelion_speed: 6.28, color: Color::srgb_u8(0, 102, 204) },
    TemplateBody { name: "Mars",    mass: 0.000000322, radius: 0.0000227, aphelion_dist: 1.666, aphelion_speed: 4.51, color: Color::srgb_u8(188, 39, 50) },
    TemplateBody { name: "Jupiter", mass: 0.000954,    radius: 0.000467,  aphelion_dist: 5.458, aphelion_speed: 2.63, color: Color::srgb_u8(218, 165, 32) },
    TemplateBody { name: "Saturn",  mass: 0.000286,    radius: 0.000395,  aphelion_dist: 10.123, aphelion_speed: 1.83, color: Color::srgb_u8(210, 180, 140) },
    TemplateBody { name: "Uranus",  mass: 0.0000437,   radius: 0.000176,  aphelion_dist: 20.11, aphelion_speed: 1.45, color: Color::srgb_u8(72, 209, 204) },
    TemplateBody { name: "Neptune", mass: 0.0000515,   radius: 0.000154,  aphelion_dist: 30.33, aphelion_speed: 1.21, color: Color::srgb_u8(0, 0, 139) },
    TemplateBody { name: "Pluto",   mass: 0.00000000658,radius: 0.00000794, aphelion_dist: 49.31, aphelion_speed: 0.67, color: Color::srgb_u8(169, 169, 169) },
];
