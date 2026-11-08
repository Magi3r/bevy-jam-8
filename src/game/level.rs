//! Spawn the main level.

use bevy::{audio::Source, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    asset_tracking::LoadResource, audio::music, game::{
        alchemy::{Recipe, RecipeBook}, animation::transition_scene, progression::Progression, tooltip,
    }, screens::{
        Screen,
        gameplay::{
            ShopArea::{self, CookingArea, Storefront, TransitionAnimation},
            storefront::storefront_scene,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub music: Handle<AudioSource>,
    #[dependency]
    pub background_storefront: Handle<Image>,
    #[dependency]
    pub deep_background_storefront: Handle<Image>,
    #[dependency]
    pub background_cooking_area: Handle<Image>,
    #[dependency]
    pub deep_background_cooking_area: Handle<Image>,
    #[dependency]
    pub book_storefront: Handle<Image>,
    #[dependency]
    pub book_cooking_area: Handle<Image>,
    #[dependency]
    pub mortar: Vec<Handle<Image>>,
    #[dependency]
    pub book_highlight_cooking_area: Handle<Image>,
    #[dependency]
    pub book_highlight_storefront: Handle<Image>,
    #[dependency]
    pub cauldron: Handle<Image>,
    #[dependency]
    pub cauldron_empty: Vec<Handle<Image>>,
    #[dependency]
    pub fire: Vec<Handle<Image>>,
    #[dependency]
    pub fire_off: Handle<Image>,
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
    pub cauldron_empty_sfx: Handle<AudioSource>,
    #[dependency]
    pub fire_ambience_sfx: Handle<AudioSource>,
    #[dependency]
    pub recipes: Handle<RecipeBook>,
    #[dependency]
    pub flask: Handle<Image>,
    #[dependency]
    pub flask_highlight: Handle<Image>,
    #[dependency]
    pub sunflower: Handle<Image>,
    #[dependency]
    pub sunflower_highlight: Handle<Image>,
    #[dependency]
    pub pull: Handle<Image>,
    #[dependency]
    pub pull_highlight: Handle<Image>,
    #[dependency]
    pub stirer: Vec<Handle<Image>>,
    #[dependency]
    pub cauldron_stir_sfx: Handle<AudioSource>,
    #[dependency]
    pub cauldron_explosion: Vec<Handle<Image>>,
    #[dependency]
    pub progression: Handle<Progression>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            // Images
            background_cooking_area: assets.load("images/background/Background_0.png"),
            deep_background_cooking_area: assets.load("images/background/DeepBackground_0.png"),
            background_storefront: assets.load("images/background/Background_1.png"),
            deep_background_storefront: assets.load("images/background/DeepBackground_1.png"),
            book_cooking_area: assets.load("images/foreground/Book_0.png"),
            book_storefront: assets.load("images/foreground/Book_1.png"),
            book_highlight_cooking_area: assets.load("images/highlight/Book_0.png"),
            book_highlight_storefront: assets.load("images/highlight/Book_1.png"),
            cauldron: assets.load("images/foreground/Cauldron.png"),
            cauldron_highlight: assets.load("images/highlight/Cauldron.png"),
            cauldron_empty: vec![
                assets.load("images/animation/cauldron_empty/cauldron0001.png"),
                assets.load("images/animation/cauldron_empty/cauldron0002.png"),
                assets.load("images/animation/cauldron_empty/cauldron0003.png"),
                assets.load("images/animation/cauldron_empty/cauldron0004.png"),
                assets.load("images/animation/cauldron_empty/cauldron0005.png"),
                assets.load("images/animation/cauldron_empty/cauldron0006.png"),
                assets.load("images/animation/cauldron_empty/cauldron0007.png"),
                assets.load("images/animation/cauldron_empty/cauldron0008.png"),
                assets.load("images/animation/cauldron_empty/cauldron0009.png"),
                assets.load("images/animation/cauldron_empty/cauldron0010.png"),
                assets.load("images/animation/cauldron_empty/cauldron0011.png"),
                assets.load("images/animation/cauldron_empty/cauldron0012.png"),
                assets.load("images/animation/cauldron_empty/cauldron0013.png"),
                assets.load("images/animation/cauldron_empty/cauldron0014.png"),
                assets.load("images/animation/cauldron_empty/cauldron0015.png"),
                assets.load("images/animation/cauldron_empty/cauldron0016.png"),
                assets.load("images/animation/cauldron_empty/cauldron0017.png"),
                assets.load("images/animation/cauldron_empty/cauldron0018.png"),
                assets.load("images/animation/cauldron_empty/cauldron0019.png"),
                assets.load("images/animation/cauldron_empty/cauldron0020.png"),
                assets.load("images/animation/cauldron_empty/cauldron0021.png"),
                assets.load("images/animation/cauldron_empty/cauldron0022.png"),
                assets.load("images/animation/cauldron_empty/cauldron0023.png"),
                assets.load("images/animation/cauldron_empty/cauldron0024.png"),
            ],
            fire: vec![
                assets.load("images/foreground/fire/Fire0001.png"),
                assets.load("images/foreground/fire/Fire0002.png"),
                assets.load("images/foreground/fire/Fire0003.png"),
                assets.load("images/foreground/fire/Fire0004.png"),
                assets.load("images/foreground/fire/Fire0005.png"),
                assets.load("images/foreground/fire/Fire0006.png"),
                assets.load("images/foreground/fire/Fire0007.png"),
                assets.load("images/foreground/fire/Fire0008.png"),
                assets.load("images/foreground/fire/Fire0009.png"),
                assets.load("images/foreground/fire/Fire0010.png"),
                assets.load("images/foreground/fire/Fire0011.png"),
                assets.load("images/foreground/fire/Fire0012.png"),
            ],
            fire_off: assets.load("images/foreground/fire/FireOff.png"),
            fire_highlight: assets.load("images/highlight/Fire.png"),
            mortar: vec![
                assets.load("images/foreground/mortar/Mortar0001.png"),
                assets.load("images/foreground/mortar/Mortar0002.png"),
                assets.load("images/foreground/mortar/Mortar0003.png"),
                assets.load("images/foreground/mortar/Mortar0004.png"),
                assets.load("images/foreground/mortar/Mortar0005.png"),
                assets.load("images/foreground/mortar/Mortar0006.png"),
                assets.load("images/foreground/mortar/Mortar0007.png"),
                assets.load("images/foreground/mortar/Mortar0008.png"),
                assets.load("images/foreground/mortar/Mortar0009.png"),
                assets.load("images/foreground/mortar/Mortar0010.png"),
                assets.load("images/foreground/mortar/Mortar0011.png"),
                assets.load("images/foreground/mortar/Mortar0012.png"),
                assets.load("images/foreground/mortar/Mortar0001.png"),
                assets.load("images/foreground/mortar/Mortar0002.png"),
                assets.load("images/foreground/mortar/Mortar0003.png"),
                assets.load("images/foreground/mortar/Mortar0004.png"),
                assets.load("images/foreground/mortar/Mortar0005.png"),
                assets.load("images/foreground/mortar/Mortar0006.png"),
                assets.load("images/foreground/mortar/Mortar0007.png"),
                assets.load("images/foreground/mortar/Mortar0008.png"),
                assets.load("images/foreground/mortar/Mortar0009.png"),
                assets.load("images/foreground/mortar/Mortar0010.png"),
                assets.load("images/foreground/mortar/Mortar0011.png"),
                assets.load("images/foreground/mortar/Mortar0012.png"),
            ],
            mortar_highlight: assets.load("images/highlight/Mortar.png"),
            transition_animation: vec![
                assets.load("images/animation/transition/transition0001.png"),
                assets.load("images/animation/transition/transition0002.png"),
                assets.load("images/animation/transition/transition0003.png"),
                assets.load("images/animation/transition/transition0004.png"),
                assets.load("images/animation/transition/transition0005.png"),
                assets.load("images/animation/transition/transition0006.png"),
                assets.load("images/animation/transition/transition0007.png"),
                assets.load("images/animation/transition/transition0008.png"),
                assets.load("images/animation/transition/transition0009.png"),
                assets.load("images/animation/transition/transition0010.png"),
                assets.load("images/animation/transition/transition0011.png"),
                assets.load("images/animation/transition/transition0012.png"),
            ],
            flask: assets.load("images/foreground/Flask.png"),
            flask_highlight: assets.load("images/highlight/Flask.png"),
            sunflower: assets.load("images/foreground/Sunflower.png"),
            sunflower_highlight: assets.load("images/highlight/Sunflower.png"),
            pull: assets.load("images/foreground/Pull.png"),
            pull_highlight: assets.load("images/highlight/Pull.png"),
            stirer: vec![
                assets.load("images/foreground/stirer/stirer0001.png"),
                assets.load("images/foreground/stirer/stirer0002.png"),
                assets.load("images/foreground/stirer/stirer0003.png"),
                assets.load("images/foreground/stirer/stirer0004.png"),
                assets.load("images/foreground/stirer/stirer0005.png"),
                assets.load("images/foreground/stirer/stirer0006.png"),
                assets.load("images/foreground/stirer/stirer0007.png"),
                assets.load("images/foreground/stirer/stirer0008.png"),
                assets.load("images/foreground/stirer/stirer0009.png"),
                assets.load("images/foreground/stirer/stirer0010.png"),
                assets.load("images/foreground/stirer/stirer0011.png"),
                assets.load("images/foreground/stirer/stirer0012.png"),
                assets.load("images/foreground/stirer/stirer0013.png"),
                assets.load("images/foreground/stirer/stirer0014.png"),
                assets.load("images/foreground/stirer/stirer0015.png"),
                assets.load("images/foreground/stirer/stirer0016.png"),
                assets.load("images/foreground/stirer/stirer0017.png"),
                assets.load("images/foreground/stirer/stirer0018.png"),
                assets.load("images/foreground/stirer/stirer0019.png"),
                assets.load("images/foreground/stirer/stirer0020.png"),
                assets.load("images/foreground/stirer/stirer0021.png"),
                assets.load("images/foreground/stirer/stirer0022.png"),
                assets.load("images/foreground/stirer/stirer0023.png"),
                assets.load("images/foreground/stirer/stirer0024.png"),
            ],
            cauldron_explosion: vec![
                assets.load("images/animation/explosion/Smoke0001.png"),
                assets.load("images/animation/explosion/Smoke0002.png"),
                assets.load("images/animation/explosion/Smoke0003.png"),
                assets.load("images/animation/explosion/Smoke0004.png"),
                assets.load("images/animation/explosion/Smoke0005.png"),
                assets.load("images/animation/explosion/Smoke0006.png"),
                assets.load("images/animation/explosion/Smoke0007.png"),
                assets.load("images/animation/explosion/Smoke0008.png"),
                assets.load("images/animation/explosion/Smoke0009.png"),
                assets.load("images/animation/explosion/Smoke0010.png"),
                assets.load("images/animation/explosion/Smoke0011.png"),
                assets.load("images/animation/explosion/Smoke0012.png"),
                assets.load("images/animation/explosion/Smoke0013.png"),
                assets.load("images/animation/explosion/Smoke0014.png"),
                assets.load("images/animation/explosion/Smoke0015.png"),
                assets.load("images/animation/explosion/Smoke0016.png"),
                assets.load("images/animation/explosion/Smoke0017.png"),
                assets.load("images/animation/explosion/Smoke0018.png"),
                assets.load("images/animation/explosion/Smoke0019.png"),
                assets.load("images/animation/explosion/Smoke0020.png"),
                assets.load("images/animation/explosion/Smoke0021.png"),
                assets.load("images/animation/explosion/Smoke0022.png"),
                assets.load("images/animation/explosion/Smoke0023.png"),
                assets.load("images/animation/explosion/Smoke0024.png"),
                assets.load("images/animation/explosion/Smoke0025.png"),
                assets.load("images/animation/explosion/Smoke0026.png"),
                assets.load("images/animation/explosion/Smoke0027.png"),
                assets.load("images/animation/explosion/Smoke0028.png"),
                assets.load("images/animation/explosion/Smoke0029.png"),
                assets.load("images/animation/explosion/Smoke0030.png"),
                assets.load("images/animation/explosion/Smoke0031.png"),
                assets.load("images/animation/explosion/Smoke0032.png"),
                assets.load("images/animation/explosion/Smoke0033.png"),
                assets.load("images/animation/explosion/Smoke0034.png"),
                assets.load("images/animation/explosion/Smoke0035.png"),
                assets.load("images/animation/explosion/Smoke0036.png"),
                assets.load("images/animation/explosion/Smoke0037.png"),
                assets.load("images/animation/explosion/Smoke0038.png"),
                assets.load("images/animation/explosion/Smoke0039.png"),
                assets.load("images/animation/explosion/Smoke0040.png"),
                assets.load("images/animation/explosion/Smoke0041.png"),
                assets.load("images/animation/explosion/Smoke0042.png"),
                assets.load("images/animation/explosion/Smoke0043.png"),
                assets.load("images/animation/explosion/Smoke0044.png"),
                assets.load("images/animation/explosion/Smoke0045.png"),
                assets.load("images/animation/explosion/Smoke0046.png"),
                assets.load("images/animation/explosion/Smoke0047.png"),
                assets.load("images/animation/explosion/Smoke0048.png"),
            ],

            // Audio
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            cauldron_bubbling_sfx: assets.load("audio/sound_effects/boiling_cauldron.mp3"),
            cauldron_empty_sfx: assets.load("audio/sound_effects/emptying_cauldron.mp3"),
            fire_ambience_sfx: assets
                .load("audio/sound_effects/Fire/engyclick-firewood-burning-sound-179862.mp3"),
            cauldron_stir_sfx: assets
                .load("audio/sound_effects/stir_couldron.mp3"),
            // Other
            recipes: assets.load("recipes.ron"),
            progression: assets.load("quests.ron"),

        }
    }
}

#[derive(Component, Default, Clone)]
pub struct FireAmbienceSFX;

#[derive(Component, Default, Clone)]
pub struct CauldronAmbienceSFX;

#[derive(Component, Default, Clone)]
pub struct LevelRoot;

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
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
        #FireAmbienceSFX
        FireAmbienceSFX
        @music(level_assets.fire_ambience_sfx.clone())
        --
        #CauldronAmbienceSFX
        CauldronAmbienceSFX
        @music(level_assets.cauldron_bubbling_sfx.clone())
        // --
        // #GameplayMusic
        // @music(level_assets.music.clone())
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
