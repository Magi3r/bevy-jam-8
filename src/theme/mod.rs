//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
    app.init_resource::<GlobalFont>();
    app.add_systems(Update, set_global_font);
    app.add_systems(Startup, load_fonts);
}

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GlobalFont(
        asset_server.load("fonts/Redaction70-Regular.ttf").into(),
    ));
}

#[derive(Resource, Clone, Default)]
struct GlobalFont(FontSource);

fn set_global_font(
    mut commands: Commands,
    query: Query<Entity, Added<Text>>,
    font: Res<GlobalFont>,
) {
    for entity in &query {
        commands.entity(entity).insert(TextFont {
            font: font.0.clone(),
            font_size: FontSize::Px(26.0),
            ..default()
        });
    }
}
