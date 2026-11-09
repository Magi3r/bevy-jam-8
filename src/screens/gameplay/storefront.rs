use bevy::prelude::*;

use crate::{
    game::{
        item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_morter,
        progression::{Npc, Progression, QuestAssets, on_click_play_quest_start_voiceline},
        tool_inventory::{MortarInventory, NPCInventory},
    },
    menus::Menu,
    screens::{Screen::Gameplay, gameplay::ShopArea},
        alchemy::{
            AlchemyItems::{self, IcyLilly, JellyNettle, Lead, Sunflower}, GetOnClick,
        }, item_scene, level::{LevelAssets, LevelRoot}, on_click_morter, progression::{Npc, Progression, QuestAssets, on_click_play_quest_start_voiceline}, tool_inventory::{MortarInventory, NPCInventory},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::Storefront),
        (
            spawn_level,
            add_herbs_based_on_progress,
            spawn_inventory_display,
            spawn_npc_inventory_display,
        )
            .chain()
            .run_if(in_state(Gameplay)),
    );
    app.add_systems(
        Update,
        update_inventory_display
            .run_if(resource_changed::<MortarInventory>)
            .run_if(in_state(ShopArea::Storefront)),
    );
    app.add_systems(
        Update,
        update_npc_inventory_display
            .run_if(resource_changed::<NPCInventory>)
            .run_if(in_state(ShopArea::Storefront)),
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
                #Book
                on(open_book_menu)
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_storefront.clone(), level_assets.book_highlight_storefront.clone())
            --
                @item_scene("Flasks".into(), "Holds one standard unit of potion.".into(), level_assets.flask.clone(), level_assets.flask_highlight.clone())
                Transform::from_xyz(0.,0.,1.5)
                GetOnClick(AlchemyItems::Flask, "images/items/Flask.png")
            --
                #Npc
                Npc
                on(on_click_play_quest_start_voiceline)
                @item_scene(progression.quests.get(progression.stage).unwrap().npc_name.clone(), progression.quests.get(progression.stage).unwrap().npc_description.clone(), quest_assets.npc_assets.get(progression.stage).unwrap().clone(), quest_assets.npc_assets.get(progression.stage).unwrap().clone())
                Transform::from_xyz(0.,0.,-0.5)
        ]
    }
}

fn open_book_menu(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Book);
}

struct HerbStruct {
    get_on_click_item: AlchemyItems,
    get_on_click_path: String,
    name: String,
    desc: String,
    path: String,
    highlight_path: String,
}

fn add_herbs_based_on_progress(
    mut commands: Commands,
    store: Single<Entity, With<StorefrontScene>>,
    progression: Res<Progression>,
    assets: Res<AssetServer>,
) {
    let stage = progression.stage;
    for i in 0..stage {
        if let Some(quest) = progression.quests.get(i) {
            for unlock in &quest.unlocks {
                if let Some(scene) = match unlock {
                    AlchemyItems::JellyNettle => Some(HerbStruct {
                        get_on_click_item: JellyNettle,
                        get_on_click_path: "images/items/JellyNettle.png".into(),
                        name: "Jelly Nettle".into(),
                        desc: "A nettle. But with magic.".into(),
                        path: "images/foreground/JellyNettle.png".into(),
                        highlight_path: "images/highlight/JellyNettle.png".into(),
                    }),
                    AlchemyItems::Sunflower => Some(HerbStruct {
                        get_on_click_item: Sunflower,
                        get_on_click_path: "images/items/Sunflower.png".into(),
                        name: "Sunflower".into(),
                        desc: "About as hot as the sun, probably.".into(),
                        path: "images/foreground/Sunflower.png".into(),
                        highlight_path: "images/highlight/Sunflower.png".into(),
                    }),
                    AlchemyItems::IcyLilly => Some(HerbStruct {
                        get_on_click_item: IcyLilly,
                        get_on_click_path: "images/items/IcyLilly.png".into(),
                        name: "Icy Lilly".into(),
                        desc: "Very cold. Mittens are advised.".into(),
                        path: "images/foreground/IcyLilly.png".into(),
                        highlight_path: "images/highlight/IcyLilly.png".into(),
                    }),
                    AlchemyItems::Lead => Some(HerbStruct {
                        get_on_click_item: Lead,
                        get_on_click_path: "images/items/Lead.png".into(),
                        name: "Lead".into(),
                        desc: "A worthless metal.".into(),
                        path: "images/foreground/Lead.png".into(),
                        highlight_path: "images/highlight/Lead.png".into(),
                    }),
                    _ => None,
                } {
                    let child = commands
                        .spawn_scene(bsn! {
                            #Herb
                            GetOnClick({scene.get_on_click_item}, {scene.get_on_click_path})
                            @item_scene({scene.name}, {scene.desc}, {assets.load(scene.path)}, {assets.load(scene.highlight_path)})
                        })
                        .id();
                    commands.entity(*store).add_child(child);
                }
            }
        }
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

#[derive(Component, Default, Clone)]
pub struct MortarInventoryList;

#[derive(Component, Default, Clone)]
pub struct NPCInventoryList;

fn spawn_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    cauldron_inv: Res<MortarInventory>,
) {
    let conatiner = commands.spawn_scene(bsn![
        #MortarInventoryList
        MortarInventoryList
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
            left: px(700),
            top: px(625),
        }
        DespawnOnExit<ShopArea>(ShopArea::Storefront)
        Pickable::IGNORE
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

fn spawn_npc_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    cauldron_inv: Res<NPCInventory>,
) {
    let conatiner = commands.spawn_scene(bsn![
        #NPCIventoryList
        NPCInventoryList
        Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            align_content: AlignContent::Start,
            flex_wrap: FlexWrap::Wrap,
            row_gap: px(2),
            column_gap: px(2),
            padding: UiRect::all(px(4)),
            width: px(300),
            height: px(500),
            left: px(200),
            top: px(175),
        }
        DespawnOnExit<ShopArea>(ShopArea::Storefront)
        Pickable::IGNORE
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
            })
            .id();

        commands.entity(container_id).add_child(id);
    }
}

fn update_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mortar_inv: Res<MortarInventory>,
    container: Query<Entity, With<MortarInventoryList>>,
) {
    if let Ok(container) = container.single() {
        commands.entity(container).despawn_children();

        for (item, quantity) in &mortar_inv.contents {
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

            commands.entity(container).add_child(id);
        }
    }
}

fn update_npc_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    npc_inv: Res<NPCInventory>,
    container: Query<Entity, With<NPCInventoryList>>,
) {
    if let Ok(container) = container.single() {
        commands.entity(container).despawn_children();

        for (item, quantity) in &npc_inv.contents {
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

            commands.entity(container).add_child(id);
        }
    }
}
