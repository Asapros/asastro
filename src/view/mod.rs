use bevy::{app::PluginGroupBuilder, prelude::*};

mod camera;
mod diagnostics;


pub(crate) struct UniverseViewPlugin;

impl PluginGroup for UniverseViewPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(camera::CameraPlugin)
            .add(diagnostics::DiagnosticsPlugin)
    }
}
