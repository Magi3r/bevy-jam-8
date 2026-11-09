use bevy::{prelude::*, reflect::display};

use crate::{
    audio::{music, sound_effect},
    game::{
        animation::FireAnimation,
        fire_scene, item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_cauldron_empty, on_click_stir_cauldron, on_click_toggle_fire_on_off,
        tool_inventory::CauldronInventory,
    },
    screens::{Screen::Gameplay, gameplay::ShopArea},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::CookingArea),
        (spawn_level, spawn_fire, spawn_inventory_display)
            .chain()
            .run_if(in_state(Gameplay)),
    );
    app.init_resource::<FireState>();
}

#[derive(Component, Clone, Default)]
pub struct Pull;

#[derive(Component, Clone, Default)]
pub struct Stirer;

#[derive(Component, Clone, Default)]
pub struct Cauldron;

#[derive(Component, Clone, Default)]
pub struct Fire;
#[derive(Component, Clone, Default)]
struct CookingAreaScene;

#[derive(Resource, Reflect)]
pub struct FireState(pub bool);

impl FromWorld for FireState {
    fn from_world(_world: &mut World) -> Self {
        FireState(true)
    }
}

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

fn spawn_fire(
    mut commands: Commands,
    scene: Query<Entity, With<CookingAreaScene>>,
    fire: Res<FireState>,
    level_assets: Res<LevelAssets>,
) {
    let fire_animation = FireAnimation::new();
    let id = if fire.0 {
        // fire on
        commands.spawn_scene(bsn!{
            #Fire
            Fire
            DespawnOnExit<ShopArea>(ShopArea::CookingArea)
            on(on_click_toggle_fire_on_off)
            fire_animation
            @fire_scene(level_assets.fire.first().unwrap().clone(), level_assets.fire_highlight.clone())
        }).id()
    } else {
        // fire off
        commands
            .spawn_scene(bsn! {
                #Fire
                Fire
                DespawnOnExit<ShopArea>(ShopArea::CookingArea)
                on(on_click_toggle_fire_on_off)
                @fire_scene(level_assets.fire_off.clone(), level_assets.fire_highlight.clone())
            })
            .id()
    };
    commands.entity(scene.single().unwrap()).add_child(id);
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

fn spawn_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    cauldron_inv: Res<CauldronInventory>,
) {
    let conatiner = commands.spawn_scene(bsn![
        #CauldronIventoryList
        Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            flex_wrap: FlexWrap::Wrap,
            row_gap: px(2),
            column_gap: px(2),
            padding: UiRect::all(px(4)),
            width: px(1000),
            height: px(250),
            left: px(0),
            top: px(520),
        }
        DespawnOnExit<ShopArea>(ShopArea::CookingArea)
    ]);

    let container_id = conatiner.id();

    for (item, quantity) in &cauldron_inv.contents {
        let item_clone = item.clone();
        let quantity_str = quantity.to_string();

        let id = commands
            .spawn_scene(bsn! {
                ImageNode {image: {assets.load(item_clone.image_path())}}
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: px(2),
                    column_gap: px(2),
                    padding: UiRect::all(px(4)),
                    width: px(125),
                    height: px(125),
                }
                Children [
                    Text(quantity_str)
                    TextFont { font_size: px(40) }
                ]
            })
            .id();

        commands.entity(container_id).add_child(id);
    }
}
