use bevy::{ecs::system::command, prelude::*, ui::widget};

use crate::{
    game::{
        alchemy::{RecipeBook, Tool},
        level::LevelAssets,
    },
    menus::Menu,
    screens::gameplay::ShopArea,
    theme::widget::{self as other_widget, button_small},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Book), (spawn_book, spawn_recipes).chain());
}

fn spawn_book(
    mut commands: Commands,
    assets: Res<AssetServer>,
    recipes: Res<Assets<RecipeBook>>,
    level_assets: Res<LevelAssets>,
    asset_server: Res<AssetServer>,
) {
    let recipe_book = recipes.get(&level_assets.recipes).unwrap();

    let button_image = asset_server.load("images/gui/ButtonSmall.png");
    let highlight = asset_server.load("images/gui/ButtonSmall_Highlight.png");

    let book = commands.spawn_scene(bsn![
        Transform::from_xyz(0.0,0.0,30.0)
            #BookRecipeList
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_wrap: FlexWrap::Wrap,
                row_gap: px(2),
                column_gap: px(2),
                padding: UiRect::all(px(4)),
            }
            DespawnOnExit<Menu>(Menu::Book)
            Visibility::Visible
            Pickable::IGNORE
            Children[
                @other_widget::button_small("X", close_book, button_image.clone(), highlight.clone())
                Node {
                    position_type: PositionType::Absolute,
                    right: px(25),
                    top: px(25),
                }
                GlobalZIndex(35)
                --
                ImageNode {
                    image: { assets.load("images/gui/Book.png") }
                }
                Pickable::IGNORE
            ]
    ]);
    //
    //     let book_container_id = book.id();
    //
    //     for recipe in &recipe_book.0 {
    //         println!("recipe name: {}", recipe.name);
    //
    //         let recipe_container = commands
    //             .spawn_scene(bsn![
    //                 Transform::from_xyz(0.0,0.0,31.0)
    //                 Children[
    //                     Node {
    //                         justify_content: JustifyContent::Center,
    //                         align_items: AlignItems::FlexStart,
    //                         flex_wrap: FlexWrap::Wrap,
    //                         row_gap: px(2),
    //                         column_gap: px(2),
    //                         padding: UiRect::all(px(4)),
    //                         width: px(600),
    //                         height: px(100),
    //                     }
    //                     DespawnOnExit<Menu>(Menu::Book)
    //                     // Pickable::IGNORE
    //                 ]
    //             ])
    //             .id();
    //
    //         commands
    //             .entity(book_container_id)
    //             .add_child(recipe_container);
    //
    //         for (ingredient, quantity) in &recipe.ingredients {
    //             let item = ingredient.as_element_or_item();
    //             let quantity_str = quantity.to_string();
    //
    //             let id = commands
    //                 .spawn_scene(bsn! {
    //                     ImageNode {image: {assets.load(item.clone().image_path())}}
    //                     Node {
    //                         justify_content: JustifyContent::Center,
    //                         align_items: AlignItems::Center,
    //                         flex_wrap: FlexWrap::Wrap,
    //                         row_gap: px(2),
    //                         column_gap: px(2),
    //                         padding: UiRect::all(px(4)),
    //                         width: px(125),
    //                         height: px(125),
    //                     }
    //                     Children [
    //                         Text(quantity_str)
    //                         TextFont { font_size: px(40) }
    //                     ]
    //                 })
    //                 .id();
    //
    //             commands.entity(recipe_container).add_child(id);

    // for input in &recipe.ingredients.clone() {
    //     println!("  - {} x{}", item.name(), qty); // uses tool_inventory's name()
    //     // or image_path() to get the icon file
    // }
    // for (output, qty) in &recipe.produces {
    //     let item = output.as_element_or_item();
    //     println!("  → {} x{}", item.name(), qty);
    // }

    //             let id = commands
    //                 .spawn_scene(bsn! {
    //                     ImageNode {image: {assets.load(item_clone.image_path())}}
    //                     Node {
    //                         justify_content: JustifyContent::Center,
    //                         align_items: AlignItems::Center,
    //                         flex_wrap: FlexWrap::Wrap,
    //                         row_gap: px(2),
    //                         column_gap: px(2),
    //                         padding: UiRect::all(px(4)),
    //                         width: px(125),
    //                         height: px(125),
    //                     }
    //                     Children [
    //                         Text(quantity_str)
    //                         TextFont { font_size: px(40) }
    //                     ]
    //                 })
    //                 .id();
    //
    //             commands.entity(container_id).add_child(id);
    // }
    // }
}

// let container_id = conatiner.id();
//
// for (item, quantity) in &cauldron_inv.contents {
//     let item_clone = item.clone();
//     let quantity_str = quantity.to_string();
//
//     let id = commands
//         .spawn_scene(bsn! {
//             ImageNode {image: {assets.load(item_clone.image_path())}}
//             Node {
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 flex_wrap: FlexWrap::Wrap,
//                 row_gap: px(2),
//                 column_gap: px(2),
//                 padding: UiRect::all(px(4)),
//                 width: px(125),
//                 height: px(125),
//             }
//             Children [
//                 Text(quantity_str)
//                 TextFont { font_size: px(40) }
//             ]
//         })
//         .id();
//
//     commands.entity(container_id).add_child(id);
// }

fn spawn_recipes(
    mut commands: Commands,
    assets: Res<AssetServer>,
    recipes: Res<Assets<RecipeBook>>,
    level_assets: Res<LevelAssets>,
    asset_server: Res<AssetServer>,
) {
    let recipe_book = recipes.get(&level_assets.recipes).unwrap();

    let conatiner = commands.spawn_scene(bsn![
        #RecipeList
        Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            flex_wrap: FlexWrap::Wrap,
            row_gap: px(2),
            column_gap: px(2),
            padding: UiRect::all(px(4)),
            width: px(1000),
            height: px(850),
            left: px(460),
            top: px(100),
        }
        DespawnOnExit<Menu>(Menu::Book)
        Pickable::IGNORE
    ]);

    let container_id = conatiner.id();

    for recipe in &recipe_book.0 {
        if !recipe.discovered {
            continue;
        }

        let recipe_container = commands
            .spawn_scene(bsn![
                #RecipeList
                Node {
                    // position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::Start,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: px(2),
                    column_gap: px(2),
                    padding: UiRect::all(px(4)),
                    width: px(480),
                    height: px(55),
                }
                DespawnOnExit<Menu>(Menu::Book)
                Pickable::IGNORE
            ])
            .id();

        commands.entity(container_id).add_child(recipe_container);

        for (ingredient, quantity) in &recipe.ingredients {
            let item = ingredient.as_element_or_item().clone();
            let quantity_str = quantity.to_string();

            let id = commands
                .spawn_scene(bsn! {
                    ImageNode {image: {assets.load(item.image_path())}}
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: px(2),
                        column_gap: px(2),
                        padding: UiRect::all(px(4)),
                        width: px(50),
                        height: px(50),
                    }
                    Children [
                        Text(quantity_str)
                        TextFont { font_size: px(40) }
                    ]
                })
                .id();
            commands.entity(recipe_container).add_child(id);
        }

        let mut path = "images/icons/Mortar.png";
        if recipe.uses == Tool::Cauldron {
            path = "images/icons/Cauldron.png";
        }

        let id = commands
            .spawn_scene(bsn! {
                ImageNode {image: {assets.load(path)}}
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: px(2),
                    column_gap: px(2),
                    padding: UiRect::all(px(4)),
                    width: px(50),
                    height: px(50),
                }
            })
            .id();
        commands.entity(recipe_container).add_child(id);

        for (product, quantity) in &recipe.produces {
            let item = product.as_element_or_item().clone();
            let quantity_str = quantity.to_string();

            let id = commands
                .spawn_scene(bsn! {
                    ImageNode {image: {assets.load(item.image_path())}}
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: px(2),
                        column_gap: px(2),
                        padding: UiRect::all(px(4)),
                        width: px(50),
                        height: px(50),
                    }
                    Children [
                        Text(quantity_str)
                        TextFont { font_size: px(40) }
                    ]
                })
                .id();
            commands.entity(recipe_container).add_child(id);
        }
    }
}

fn close_book(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
