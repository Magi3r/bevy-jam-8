//! Spawn the main level.

use bevy::prelude::*;
use bevy_inspector_egui::egui::Tooltip;

use crate::{
    asset_tracking::LoadResource, audio::music, demo::{
        level,
        player::{PlayerAssets, player},
    }, game::{Interactable, animation::FireAnimation, fire_scene, item_scene, tooltip}, screens::{Screen, gameplay::{cooking_area::cooking_area_scene, storefront::storefront_scene}},
};


pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub music: Handle<AudioSource>,
    #[dependency]
    pub background_storefront: Handle<Image>,
    #[dependency]
    pub background_cooking_area: Handle<Image>,
    #[dependency]
    pub book_storefront: Handle<Image>,
    #[dependency]
    pub book_cooking_area: Handle<Image>,
    #[dependency]
    pub mortar: Handle<Image>,
    #[dependency]
    pub book_highlight_cooking_area: Handle<Image>,
    #[dependency]
    pub book_highlight_storefront: Handle<Image>,
    #[dependency]
    pub cauldron: Handle<Image>,
    #[dependency]
    pub fire: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            background_cooking_area: assets.load("images/background/Background_0.png"),
            background_storefront: assets.load("images/background/Background_1.png"),
            book_cooking_area: assets.load("images/foreground/Book_0.png"),
            book_storefront: assets.load("images/foreground/Book_1.png"),
            book_highlight_cooking_area: assets.load("images/highlight/Book_0.png"),
            book_highlight_storefront: assets.load("images/highlight/Book_1.png"),
            cauldron: assets.load("images/foreground/Cauldron.png"),
            fire: assets.load("images/foreground/FireAtlas.png"),
            mortar: assets.load("images/foreground/Mortar.png"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn_scene(bsn! {
        #Level
        Transform
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children [
            // @cooking_area_scene(&level_assets, &mut texture_atlas_layouts)
            @storefront_scene(&level_assets, &mut texture_atlas_layouts)

            --
            #GameplayMusic
            @music(level_assets.music.clone())
        ]
    });
    commands.spawn_scene(bsn! {
        @tooltip()
        DespawnOnExit<Screen>(Screen::Gameplay)
    });
}

fn switch_scene_on_input(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        todo!("Switch Scene left");
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        todo!("Switch Scene right");
    }

}
