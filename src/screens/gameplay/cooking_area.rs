use std::time::Duration;

use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*, reflect::display};

use crate::{
    game::{
        alchemy::{Draggable, Element, HeldItem},
        animation::FireAnimation,
        fire_scene, item_scene,
        level::{LevelAssets, LevelRoot},
        on_click_cauldron_empty, on_click_stir_cauldron, on_click_toggle_fire_on_off,
        tool_inventory::{CauldronInventory, ElementOrItem},
    },
    menus::Menu,
    screens::{
        Screen::Gameplay,
        gameplay::{ShopArea, storefront::alchemy_item_to_herb_struct},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(ShopArea::CookingArea),
        (spawn_level, spawn_fire, spawn_inventory_display, update_inventory_display)
            .chain()
            .run_if(in_state(Gameplay)),
    );
    app.add_systems(
        Update,
        update_inventory_display
            .run_if(resource_changed::<CauldronInventory>)
            .run_if(in_state(ShopArea::CookingArea)),
    );
    app.add_systems(Update, update_fire_add);
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
                on(open_book_menu)
                @item_scene("Book".to_string(), "Study discovered recipes".to_string(), level_assets.book_cooking_area.clone(), level_assets.book_highlight_cooking_area.clone())
            --
                #Pull
                Pull
                on(on_click_cauldron_empty)
                @item_scene("Pull".to_string(), "Pull to flush".to_string(), level_assets.pull.clone(), level_assets.pull_highlight.clone())
            --
                FireAdd
        ]
    }
}

fn open_book_menu(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Book);
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct FireAdd {
    timer: f32,
}

fn update_fire_add(
    mut query: Query<&mut FireAdd>,
    fire: Res<FireState>,
    mut cauldron: ResMut<CauldronInventory>,
    time: Res<Time>,
) {
    if !fire.0 {
        return;
    }
    for mut fire_add in query {
        fire_add.timer += time.delta().as_secs_f32();

        if fire_add.timer < 1.0 {
            return;
        }

        fire_add.timer = 0.0;

        let index = cauldron
            .contents
            .iter()
            .position(|(name, _)| *name == ElementOrItem::Element(Element::Fire));

        if let Some(i) = index {
            if cauldron.contents[i].1 >= 10 {
                return;
            }
            cauldron.contents[i].1 += 1;
        } else {
            cauldron
                .contents
                .push((ElementOrItem::Element(Element::Fire), 1))
        }
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

#[derive(Component, Default, Clone)]
pub struct CauldronInventoryList;

fn spawn_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    cauldron_inv: Res<CauldronInventory>,
) {
    let conatiner = commands.spawn_scene(bsn! {
        #CauldronInventoryList
        CauldronInventoryList
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

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
struct ItemInCauldron(ElementOrItem, usize);

fn update_inventory_display(
    mut commands: Commands,
    assets: Res<AssetServer>,
    cauldron_inv: Res<CauldronInventory>,
    container: Query<Entity, With<CauldronInventoryList>>,
) {
    if let Ok(container) = container.single() {
        commands.entity(container).despawn_children();

        for (item, quantity) in &cauldron_inv.contents {
            let item_clone = item.clone();
            let quantity_str = quantity.to_string();

            let id = commands
                .spawn_scene(bsn! {
                    ItemInCauldron({item.clone()}, {quantity.clone()})
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
                        Pickable::IGNORE
                    ]
                    on(on_click_get_item_from_cauldron)
                })
                .id();

            commands.entity(container).add_child(id);
        }
    }
}

fn on_click_get_item_from_cauldron(
    event: On<PointerClick>,
    mut held_item: ResMut<HeldItem>,
    mut inv: ResMut<CauldronInventory>,
    mut item_in_cauldron: Query<&mut ItemInCauldron>,
    mut commands: Commands,
) {
    if held_item.entity.is_none() {
        if let Ok(ItemInCauldron(ElementOrItem::Item(item), _)) = item_in_cauldron.get(event.entity)
        {
            let item_struct = alchemy_item_to_herb_struct(item);
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
