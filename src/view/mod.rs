use bevy::{app::PluginGroupBuilder, prelude::*};

mod movement;
mod diagnostics;
pub mod follow;

pub(crate) struct UniverseViewPlugin;

impl PluginGroup for UniverseViewPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(movement::CameraMovementPlugin)
            .add(diagnostics::DiagnosticsPlugin)
            .add(follow::CameraFollowPlugin)
    }
}
