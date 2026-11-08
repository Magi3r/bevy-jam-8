use bevy::prelude::*;

use crate::{
    game::{
        item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_morter,
        progression::{Npc, Progression, QuestAssets, on_click_play_quest_start_voiceline},
    },
    screens::{Screen::Gameplay, gameplay::ShopArea},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::Storefront),
        spawn_level.run_if(in_state(Gameplay)),
    );
}

#[derive(Component, Clone, Default)]
pub struct Mortar;

#[derive(Component, Clone, Default)]
struct StorefrontScene;

pub fn storefront_scene(
    level_assets: &Res<LevelAssets>,
    quest_assets: &Res<QuestAssets>,
    progression: &Res<Progression>,
) -> impl Scene {
    bsn! {
        #Storefront
        StorefrontScene
        Sprite {image: {level_assets.background_storefront.clone()}}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<ShopArea>(ShopArea::Storefront)
        Children [
                Sprite {image: {level_assets.deep_background_storefront.clone()}}
                Transform::from_xyz(0.0,0.0,-2.0)
            --
                #Mortar
                Mortar
                on(on_click_morter)
                @item_scene("Mortar".to_string(), "Grind stuff into other stuff".to_string(), level_assets.mortar.first().unwrap().clone(), level_assets.mortar_highlight.clone())
            --
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_storefront.clone(), level_assets.book_highlight_storefront.clone())
            --
                @item_scene("Flasks".into(), "Holds one standard unit of potion.".into(), level_assets.flask.clone(), level_assets.flask_highlight.clone())
                Transform::from_xyz(0.,0.,1.5)
            --
                #Sunflower
                @item_scene("Sunflower".into(), "About as hot as the sun, probably.".into(), level_assets.sunflower.clone(), level_assets.sunflower_highlight.clone())
            --
                #Npc
                Npc
                on(on_click_play_quest_start_voiceline)
                @item_scene(progression.quests.get(progression.stage).unwrap().npc_name.clone(), progression.quests.get(progression.stage).unwrap().npc_description.clone(), quest_assets.npc_assets.get(progression.stage).unwrap().clone(), quest_assets.npc_assets.get(progression.stage).unwrap().clone())
                Transform::from_xyz(0.,0.,-0.5)
        ]
    }
}

fn spawn_level(
    mut commands: Commands,
    level: Query<Entity, With<LevelRoot>>,
    level_assets: Res<LevelAssets>,
    quest_assets: Res<QuestAssets>,
    progression: Res<Progression>,
) {
    for level in level {
        let child = commands
            .spawn_scene(bsn! {
                @storefront_scene(&level_assets, &quest_assets, &progression)
            })
            .id();
        commands.entity(level).add_child(child);
    }
}
