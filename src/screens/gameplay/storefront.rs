use bevy::prelude::*;

use crate::{
    game::{
        item_scene,
        level::{LevelAssets, LevelRoot},
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
struct StorefrontScene;

pub fn storefront_scene(
    level_assets: &Res<LevelAssets>,
) -> impl Scene {
    bsn! {
        #Storefront
        StorefrontScene
        Sprite {image: {level_assets.background_storefront.clone()}}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<ShopArea>(ShopArea::Storefront)
        Children [
                @item_scene("Mortar".to_string(), "Grind stuff into other stuff".to_string(), level_assets.mortar.clone(), level_assets.mortar_highlight.clone())
            --
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_storefront.clone(), level_assets.book_highlight_storefront.clone())
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
                @storefront_scene(&level_assets)
            })
            .id();
        commands.entity(level).add_child(child);
    }
}
