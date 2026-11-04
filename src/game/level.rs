//! Spawn the main level.

use bevy::{audio::Source, prelude::*};

use crate::{
    asset_tracking::LoadResource, audio::music, game::{animation::transition_scene, tooltip}, screens::{Screen, gameplay::{ShopArea::{CookingArea, Storefront}, storefront::storefront_scene}},
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
    pub fire: Vec<Handle<Image>>,
    #[dependency]
    pub transition_animation: Vec<Handle<Image>>,
    #[dependency]
    pub mortar_highlight: Handle<Image>,
    #[dependency]
    pub fire_highlight: Handle<Image>,
    #[dependency]
    pub cauldron_highlight: Handle<Image>,
    #[dependency]
    pub cauldron_bubbling_sfx: Handle<AudioSource>,
    #[dependency]
    pub fire_ambience_sfx: Handle<AudioSource>
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
            cauldron_highlight: assets.load("images/highlight/Cauldron.png"),
            fire: vec![
                assets.load("images/foreground/Fire0001.png"),
                assets.load("images/foreground/Fire0002.png"),
                assets.load("images/foreground/Fire0003.png"),
                assets.load("images/foreground/Fire0004.png"),
                assets.load("images/foreground/Fire0005.png"),
                assets.load("images/foreground/Fire0006.png"),
                assets.load("images/foreground/Fire0007.png"),
                assets.load("images/foreground/Fire0008.png"),
                assets.load("images/foreground/Fire0009.png"),
                assets.load("images/foreground/Fire0010.png"),
                assets.load("images/foreground/Fire0009.png"),
                assets.load("images/foreground/Fire0008.png"),
                assets.load("images/foreground/Fire0007.png"),
                assets.load("images/foreground/Fire0006.png"),
                assets.load("images/foreground/Fire0005.png"),
                assets.load("images/foreground/Fire0004.png"),
                assets.load("images/foreground/Fire0003.png"),
                assets.load("images/foreground/Fire0002.png"),
            ],
            fire_highlight: assets.load("images/highlight/Fire.png"),
            mortar: assets.load("images/foreground/Mortar.png"),
            mortar_highlight: assets.load("images/highlight/Mortar.png"),
            transition_animation: vec![
                assets.load("images/animation/transition0001.png"),
                assets.load("images/animation/transition0002.png"),
                assets.load("images/animation/transition0003.png"),
                assets.load("images/animation/transition0004.png"),
                assets.load("images/animation/transition0005.png"),
                assets.load("images/animation/transition0006.png"),
                assets.load("images/animation/transition0007.png"),
                assets.load("images/animation/transition0008.png"),
                assets.load("images/animation/transition0009.png"),
                assets.load("images/animation/transition0010.png"),
                assets.load("images/animation/transition0011.png"),
                assets.load("images/animation/transition0012.png"),
            ],

            // Audio
            cauldron_bubbling_sfx: assets.load("audio/sound_effects/boiling_cauldron.mp3"),
            fire_ambience_sfx: assets.load("audio/sound_effects/Fire/engyclick-firewood-burning-sound-179862.mp3"),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct LevelRoot;

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn_scene(bsn! {
        #LevelRoot
        LevelRoot
        Transform
        Visibility
        DespawnOnExit<Screen>(Screen::Gameplay)
        Children [
            // @cooking_area_scene(&level_assets)
            // @storefront_scene(&level_assets)
            @transition_scene(true, Storefront, &level_assets)

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

fn switch_scene_on_input(input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        todo!("Switch Scene left");
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        todo!("Switch Scene right");
    }
}
