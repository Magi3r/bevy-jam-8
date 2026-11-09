use bevy::prelude::*;

use crate::{
    game::{
        alchemy::{
            AlchemyItems::{
                self, BasicPotion, Cheese, EternalSnow, Flask, Gold, HealthPotion, IcyLilly,
                IcyPowder, JellyNettle, JellyPowder, Lead, MoltenSnowman, OrbBroken, OrbClear,
                OrbUnclear, PhilosopherStone, Snowman, Sunflower, SunflowerPowder,
            },
            Draggable, GetOnClick, HeldItem,
        },
        item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_morter,
        progression::{Npc, Progression, QuestAssets, on_click_play_quest_start_voiceline},
        tool_inventory::{ElementOrItem, MortarInventory, NPCInventory},
    },
    menus::Menu,
    screens::{Screen::Gameplay, gameplay::ShopArea},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::Storefront),
        (
            spawn_level,
            add_herbs_based_on_progress,
            spawn_inventory_display,
            update_inventory_display,
            spawn_npc_inventory_display,
             update_npc_inventory_display
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

pub struct HerbStruct {
    pub get_on_click_item: AlchemyItems,
    pub get_on_click_path: String,
    pub name: String,
    pub desc: String,
    pub path: String,
    pub highlight_path: String,
}

pub fn alchemy_item_to_herb_struct(item: &AlchemyItems) -> HerbStruct {
    match item {
        AlchemyItems::JellyNettle => HerbStruct {
            get_on_click_item: JellyNettle,
            get_on_click_path: "images/items/JellyNettle.png".into(),
            name: "Jelly Nettle".into(),
            desc: "A nettle. But with magic.".into(),
            path: "images/foreground/JellyNettle.png".into(),
            highlight_path: "images/highlight/JellyNettle.png".into(),
        },
        AlchemyItems::Sunflower => HerbStruct {
            get_on_click_item: Sunflower,
            get_on_click_path: "images/items/Sunflower.png".into(),
            name: "Sunflower".into(),
            desc: "About as hot as the sun, probably.".into(),
            path: "images/foreground/Sunflower.png".into(),
            highlight_path: "images/highlight/Sunflower.png".into(),
        },
        AlchemyItems::IcyLilly => HerbStruct {
            get_on_click_item: IcyLilly,
            get_on_click_path: "images/items/IcyLilly.png".into(),
            name: "Icy Lilly".into(),
            desc: "Very cold. Mittens are advised.".into(),
            path: "images/foreground/IcyLilly.png".into(),
            highlight_path: "images/highlight/IcyLilly.png".into(),
        },
        AlchemyItems::Lead => HerbStruct {
            get_on_click_item: Lead,
            get_on_click_path: "images/items/Lead.png".into(),
            name: "Lead".into(),
            desc: "A worthless metal.".into(),
            path: "images/foreground/Lead.png".into(),
            highlight_path: "images/highlight/Lead.png".into(),
        },
        AlchemyItems::OrbClear => HerbStruct {
            get_on_click_item: OrbClear,
            get_on_click_path: "images/items/OrbClear.png".into(),
            name: "Clear Orb".into(),
            desc: "Perfectly clear again.".into(),
            path: "images/foreground/OrbClear.png".into(),
            highlight_path: "images/highlight/OrbClear.png".into(),
        },
        AlchemyItems::OrbUnclear => HerbStruct {
            get_on_click_item: OrbUnclear,
            get_on_click_path: "images/items/OrbUnclear.png".into(),
            name: "Milky Orb".into(),
            desc: "You can't see anything through it.".into(),
            path: "images/foreground/OrbUnclear.png".into(),
            highlight_path: "images/highlight/OrbUnclear.png".into(),
        },
        AlchemyItems::OrbBroken => HerbStruct {
            get_on_click_item: OrbBroken,
            get_on_click_path: "images/items/OrbBroken.png".into(),
            name: "Shattered Orb".into(),
            desc: "Has sharp edges.".into(),
            path: "images/foreground/OrbBroken.png".into(),
            highlight_path: "images/highlight/OrbBroken.png".into(),
        },
        AlchemyItems::JellyPowder => HerbStruct {
            get_on_click_item: JellyPowder,
            get_on_click_path: "images/items/JellyPowder.png".into(),
            name: "Jelly Powder".into(),
            desc: "Fine herbal powder.".into(),
            path: "images/foreground/JellyPowder.png".into(),
            highlight_path: "images/highlight/JellyPowder.png".into(),
        },
        AlchemyItems::IcyPowder => HerbStruct {
            get_on_click_item: IcyPowder,
            get_on_click_path: "images/items/IcyPowder.png".into(),
            name: "Jelly Powder".into(),
            desc: "Cold herbal powder.".into(),
            path: "images/foreground/IcyPowder.png".into(),
            highlight_path: "images/highlight/IcyPowder.png".into(),
        },
        AlchemyItems::SunflowerPowder => HerbStruct {
            get_on_click_item: SunflowerPowder,
            get_on_click_path: "images/items/SunflowerPowder.png".into(),
            name: "Sunflower Powder".into(),
            desc: "Hot herbal powder.".into(),
            path: "images/foreground/SunflowerPowder.png".into(),
            highlight_path: "images/highlight/SunflowerPowder.png".into(),
        },
        AlchemyItems::HealthPotion => HerbStruct {
            get_on_click_item: HealthPotion,
            get_on_click_path: "images/items/HealthPotion.png".into(),
            name: "Healing Potion".into(),
            desc: "A healthy potion.".into(),
            path: "images/foreground/HealthPotion.png".into(),
            highlight_path: "images/highlight/HealthPotion.png".into(),
        },
        AlchemyItems::BasicPotion => HerbStruct {
            get_on_click_item: BasicPotion,
            get_on_click_path: "images/items/BasicPotion.png".into(),
            name: "Basic Potion".into(),
            desc: "As healthy as hot water.".into(),
            path: "images/foreground/BasicPotion.png".into(),
            highlight_path: "images/highlight/BasicPotion.png".into(),
        },
        AlchemyItems::MoltenSnowman => HerbStruct {
            get_on_click_item: MoltenSnowman,
            get_on_click_path: "images/items/MoltenSnowman.png".into(),
            name: "Molten Snowman".into(),
            desc: "Looks very sad.".into(),
            path: "images/foreground/MoltenSnowman.png".into(),
            highlight_path: "images/highlight/MoltenSnowman.png".into(),
        },
        AlchemyItems::Snowman => HerbStruct {
            get_on_click_item: Snowman,
            get_on_click_path: "images/items/Snowman.png".into(),
            name: "Snowman".into(),
            desc: "Will never melt again.".into(),
            path: "images/foreground/Snowman.png".into(),
            highlight_path: "images/highlight/Snowman.png".into(),
        },
        AlchemyItems::EternalSnow => HerbStruct {
            get_on_click_item: EternalSnow,
            get_on_click_path: "images/items/EternalSnow.png".into(),
            name: "Eternal Snow".into(),
            desc: "Snow that can withstand intense heat.".into(),
            path: "images/foreground/EternalSnow.png".into(),
            highlight_path: "images/highlight/EternalSnow.png".into(),
        },
        AlchemyItems::Flask => HerbStruct {
            get_on_click_item: Flask,
            get_on_click_path: "images/items/Flask.png".into(),
            name: "Flask".into(),
            desc: "Holds liquid.".into(),
            path: "images/foreground/Flask.png".into(),
            highlight_path: "images/highlight/Flask.png".into(),
        },
        AlchemyItems::PhilosopherStone => HerbStruct {
            get_on_click_item: PhilosopherStone,
            get_on_click_path: "images/items/PhilosopherStone.png".into(),
            name: "Philosophers Stone".into(),
            desc: "A bit overrated, but useful indeed.".into(),
            path: "images/foreground/PhilosopherStone.png".into(),
            highlight_path: "images/highlight/PhilosopherStone.png".into(),
        },
        AlchemyItems::Gold => HerbStruct {
            get_on_click_item: Gold,
            get_on_click_path: "images/items/Gold.png".into(),
            name: "Gold".into(),
            desc: "Gold. Somehow everbody wants it.".into(),
            path: "images/foreground/Gold.png".into(),
            highlight_path: "images/highlight/Gold.png".into(),
        },
        AlchemyItems::Cheese => HerbStruct {
            get_on_click_item: Cheese,
            get_on_click_path: "images/items/Cheese.png".into(),
            name: "Cheese".into(),
            desc: "Edible gold with holes.".into(),
            path: "images/foreground/Cheese.png".into(),
            highlight_path: "images/highlight/Cheese.png".into(),
        },
    }
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
                    AlchemyItems::JellyNettle => Some(alchemy_item_to_herb_struct(unlock)),
                    AlchemyItems::Sunflower => Some(alchemy_item_to_herb_struct(unlock)),
                    AlchemyItems::IcyLilly => Some(alchemy_item_to_herb_struct(unlock)),
                    AlchemyItems::Lead => Some(alchemy_item_to_herb_struct(unlock)),
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
    let conatiner = commands.spawn_scene(bsn! {
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
    });

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
    let conatiner = commands.spawn_scene(bsn! {
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
    });

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

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
pub struct ItemInMortar(pub ElementOrItem, pub usize);

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
pub struct ItemInNPC(pub ElementOrItem, pub usize);

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
                    ItemInMortar({item.clone()}, {quantity.clone()})
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
                        Pickable::IGNORE
                    ]
                     on(on_click_get_item_from_mortar)
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
                    ItemInNPC({item.clone()}, {quantity.clone()})
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
                    // Children [
                    //     Text(quantity_str)
                    //     TextFont { font_size: px(40) }
                    // ]
                     on(on_click_get_item_from_npc)
                })
                .id();

            commands.entity(container).add_child(id);
        }
    }
}

fn on_click_get_item_from_mortar(
    event: On<PointerClick>,
    mut held_item: ResMut<HeldItem>,
    mut inv: ResMut<MortarInventory>,
    mut item_in_cauldron: Query<&mut ItemInMortar>,
    mut commands: Commands,
) {
    if held_item.entity.is_none() {
        if let Ok(ItemInMortar(ElementOrItem::Item(item), _)) = item_in_cauldron.get(event.entity) {
            let item_struct = alchemy_item_to_herb_struct(&item);
            let id = commands
                .spawn_scene(bsn! {
                    Draggable
                    Sprite {image: {item_struct.get_on_click_path.clone()} }
                    Transform::from_xyz(0.,0.,50.).with_scale((0.2,0.2,1.0).into())
                })
                .id();
            held_item.entity = Some(id);
            held_item.item = Some(item.clone());
            let mut i = 0;
            let mut remove = None;
            for (cont, quant) in &mut inv.contents {
                if *cont == ElementOrItem::Item(item.clone()) {
                    if *quant - 1 == 0 {
                        remove = Some(i);
                    } else {
                        *quant -= 1;
                    }
                }
                i += 1;
            }
            if let Some(remove) = remove {
                inv.contents.remove(remove);
            }
        }
    }
}
fn on_click_get_item_from_npc(
    event: On<PointerClick>,
    mut held_item: ResMut<HeldItem>,
    mut inv: ResMut<NPCInventory>,
    mut item_in_cauldron: Query<&mut ItemInNPC>,
    mut commands: Commands,
) {
    if held_item.entity.is_none() {
        if let Ok(ItemInNPC(ElementOrItem::Item(item), _)) = item_in_cauldron.get(event.entity) {
            let item_struct = alchemy_item_to_herb_struct(&item);
            let id = commands
                .spawn_scene(bsn! {
                    Draggable
                    Sprite {image: {item_struct.get_on_click_path.clone()} }
                    Transform::from_xyz(0.,0.,50.).with_scale((0.2,0.2,1.0).into())
                })
                .id();
            held_item.entity = Some(id);
            held_item.item = Some(item.clone());
            let mut i = 0;
            let mut remove = None;
            for (cont, quant) in &mut inv.contents {
                if *cont == ElementOrItem::Item(item.clone()) {
                    if *quant - 1 == 0 {
                        remove = Some(i);
                    } else {
                        *quant -= 1;
                    }
                }
                i += 1;
            }
            if let Some(remove) = remove {
                inv.contents.remove(remove);
            }
        }
    }
}
