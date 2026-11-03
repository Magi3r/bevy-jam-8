use bevy::prelude::*;

use crate::{
    game::{
        fire_scene, item_scene,
        level::{LevelAssets, LevelRoot},
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
struct CookingAreaScene;

pub fn cooking_area_scene(
    level_assets: &Res<LevelAssets>,
    mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> impl Scene {
    bsn! {
        #CookingArea
        CookingAreaScene
        Sprite {image: {level_assets.background_cooking_area.clone()}}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<ShopArea>(ShopArea::CookingArea)
        Children [
                @fire_scene(level_assets.fire.clone(), &mut texture_atlas_layouts)
            --
                @item_scene("Cauldron".to_string(), "Put in items and hope for the best".to_string(), level_assets.cauldron.clone())
            --
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_cooking_area.clone())
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
                @cooking_area_scene(&level_assets, &mut texture_atlas_layouts)
            })
            .id();
        commands.entity(level).add_child(child);
    }
}
