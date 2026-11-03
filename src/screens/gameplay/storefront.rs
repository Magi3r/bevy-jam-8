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
    _texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> impl Scene {
    bsn! {
        #Storefront
        StorefrontScene
        Sprite {image: {level_assets.background_storefront.clone()}}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<ShopArea>(ShopArea::Storefront)
        Children [
                @item_scene("Mortar".to_string(), "Grind stuff into other stuff".to_string(), level_assets.mortar.clone())
            --
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_storefront.clone())
        ]
    }
}

fn spawn_level(
    mut commands: Commands,
    level: Query<Entity, With<LevelRoot>>,
    level_assets: Res<LevelAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for level in level {
        let child = commands
            .spawn_scene(bsn! {
                @storefront_scene(&level_assets, &mut texture_atlas_layouts)
            })
            .id();
        commands.entity(level).add_child(child);
    }
}
