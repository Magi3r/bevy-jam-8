//! Spawn the main level.

use bevy::{audio::Source, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    game::{
        alchemy::{Recipe, RecipeBook},
        animation::transition_scene,
        tooltip,
    },
    screens::{
        Screen,
        gameplay::{
            ShopArea::{self, CookingArea, Storefront, TransitionAnimation},
            storefront::storefront_scene,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<RecipeBook>();
    app.add_plugins(RonAssetPlugin::<RecipeBook>::new(&["ron"]));
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
    pub cauldron_empty: Vec<Handle<Image>>,
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
    pub fire_ambience_sfx: Handle<AudioSource>,
    #[dependency]
    pub recipes: Handle<RecipeBook>,
    #[dependency]
    pub flask: Handle<Image>,
    #[dependency]
    pub flask_highlight: Handle<Image>,

}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            // Images
            background_cooking_area: assets.load("images/background/Background_0.png"),
            background_storefront: assets.load("images/background/Background_1.png"),
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
            fire_highlight: assets.load("images/highlight/Fire.png"),
            mortar: assets.load("images/foreground/Mortar.png"),
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

            // Audio
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            cauldron_bubbling_sfx: assets.load("audio/sound_effects/boiling_cauldron.mp3"),
            fire_ambience_sfx: assets
                .load("audio/sound_effects/Fire/engyclick-firewood-burning-sound-179862.mp3"),
            // Other
            recipes: assets.load("recipes.ron"),
        }
    }
}

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
        @music(level_assets.fire_ambience_sfx.clone())
        --
        #CauldronAmbienceSFX
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
