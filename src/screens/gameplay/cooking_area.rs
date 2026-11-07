use bevy::prelude::*;

use crate::{
    audio::{music, sound_effect},
    game::{
        fire_scene, item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_cauldron_empty, on_click_stir_cauldron, on_click_toggle_fire_on_off,
    },
    screens::{Screen::Gameplay, gameplay::ShopArea},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::CookingArea),
        spawn_level.run_if(in_state(Gameplay)),
    );
}

#[derive(Component, Clone, Default)]
pub struct Pull;

#[derive(Component, Clone, Default)]
pub struct Stirer;

#[derive(Component, Clone, Default)]
pub struct Cauldron;

#[derive(Component, Clone, Default)]
pub struct Fire(pub bool);
#[derive(Component, Clone, Default)]
struct CookingAreaScene;

pub fn cooking_area_scene(level_assets: &Res<LevelAssets>) -> impl Scene {
    bsn! {
        #CookingArea
        CookingAreaScene
        Sprite {image: {level_assets.background_cooking_area.clone()}}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<ShopArea>(ShopArea::CookingArea)
        Children [
                Sprite {image: {level_assets.deep_background_cooking_area.clone()}}
                Transform::from_xyz(0.0,0.0,-2.0)
            --
                #Fire
                Fire(true)
                on(on_click_toggle_fire_on_off)
                @fire_scene(level_assets.fire.first().unwrap().clone(), level_assets.fire_highlight.clone())
            --
                #Cauldron
                Cauldron
                on(on_click_stir_cauldron)
                @item_scene("Cauldron".to_string(), "Put in items and hope for the best".to_string(), level_assets.cauldron.clone(), level_assets.cauldron_highlight.clone())
            --
                #Stirer
                Stirer
                Sprite {image: {level_assets.stirer.first().unwrap().clone()}}
                Transform::from_xyz(0.0,0.0,2.0)
            --
                #Book
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_cooking_area.clone(), level_assets.book_highlight_cooking_area.clone())
            --
                #Pull
                Pull
                on(on_click_cauldron_empty)
                @item_scene("Pull".to_string(), "Pull to flush".to_string(), level_assets.pull.clone(), level_assets.pull_highlight.clone())
        ]
    }
}

fn spawn_level(
    mut commands: Commands,
    level: Query<Entity, With<LevelRoot>>,
    level_assets: Res<LevelAssets>,
) {
    for level in level {
        let child = commands
            .spawn_scene(bsn! {
                @cooking_area_scene(&level_assets)
            })
            .id();
        commands.entity(level).add_child(child);
    }
}
