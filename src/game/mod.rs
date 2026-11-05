use bevy::{
    ecs::template::OptionTemplate, image::TextureAtlasTemplate, picking::hover::Hovered, prelude::*, window::PrimaryWindow,
};

use crate::{
    game::animation::FireAnimation, screens::Screen::Gameplay, theme::palette::BUTTON_BACKGROUND,
};

const TOOLTIP_OFFSET: Vec2 = Vec2::new(15.0, 15.0);

pub mod alchemy;
pub mod animation;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((alchemy::plugin, level::plugin, animation::plugin));
    app.add_systems(
        Update,
        (
            on_hover_toggle_item_tooltip,
            update_tooltip_position,
            on_hover_update_tooltip_text,
            on_hover_show_highlight,
        )
            .run_if(in_state(Gameplay)),
    );
    // #[cfg(feature = "dev")]
    // app.add_systems(Update, debug_items);
}

#[derive(Component)]
struct Machine;
#[derive(Component, Default, Clone)]
struct Interactable {
    name: String,
    description: String,
}

#[derive(Component, Default, Clone)]
struct Tooltip;

#[derive(Component, Default, Clone)]
struct TooltipTitle;

#[derive(Component, Default, Clone)]
struct Highlight;

#[derive(Component, Default, Clone)]
struct TooltipDescription;

pub fn item_scene(
    name: String,
    description: String,
    image: Handle<Image>,
    highlight_image: Handle<Image>,
) -> impl Scene {
    bsn! {
        #Item
        Interactable {
            name: { name.clone() },
            description: { description.clone() },
        }
        Sprite {image}
        Transform::from_xyz(0.0,0.0,1.0)
        Pickable
        Hovered
        Children [
            @highlight(highlight_image)
        ]
    }
}

/// The fire below the cauldron.
pub fn fire_scene(image: Handle<Image>, highlight_image: Handle<Image>) -> impl Scene {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let fire_animation = FireAnimation::new();
    bsn! {
        #CauldronFire
        fire_animation
        Interactable {name: "Fire", description: "Ouch! Its hot. Why not touch it again?"}
        Sprite {
            image:  { image },
        }
        Transform::from_xyz(0.0,0.0,0.0)
        Children [
            @highlight(highlight_image)
        ]
        Pickable
        Hovered
    }
}

fn tooltip() -> impl Scene {
    bsn! {
        #Tooltip
        Tooltip
        Node {
            position_type: PositionType::Absolute,
            // width: px(200),
            // height: px(60),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(10)),
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
        }
        BackgroundColor(BUTTON_BACKGROUND)
        Children [
            #ItemName
            TooltipTitle
            Text({"#ItemName".to_owned()})
            --
            #ItemDescription
            TooltipDescription
            Text({"#ItemDescription".to_owned()})
        ]
        Visibility::Hidden
        GlobalZIndex(100)
    }
}

fn highlight(image: Handle<Image>) -> impl Scene {
    bsn! {
        #Highlight
        Highlight
        Sprite {
            image,
        }
        Transform::from_xyz(0.0, 0.0, -0.1)
        Visibility::Hidden
    }
}

fn on_hover_update_tooltip_text(
    items: Query<(&Hovered, &Interactable), Changed<Hovered>>,
    mut title: Query<&mut Text, (With<TooltipTitle>, Without<TooltipDescription>)>,
    mut description: Query<&mut Text, With<TooltipDescription>>,
) {
    for (hovered, item) in items {
        if hovered.0 {
            **title.single_mut().unwrap() = item.name.clone();
            **description.single_mut().unwrap() = item.description.clone();
        }
    }
}

fn on_hover_toggle_item_tooltip(
    items: Query<&Hovered, (With<Interactable>, Changed<Hovered>)>,
    mut tooltips: Query<&mut Visibility, With<Tooltip>>,
) {
    for hovered in items {
        if let Ok(mut visibility) = tooltips.single_mut() {
            // visibility.toggle_visible_hidden();
            *visibility = match hovered.get() {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            }
        }
    }
}

fn update_tooltip_position(
    mut messages: MessageReader<CursorMoved>,
    mut tooltip: Query<(&mut Node, &ComputedNode), With<Tooltip>>,
    window: Query<&Window>,
) {
    if let Some(message) = messages.read().last() {
        let pos = message.position;
        if let Ok((mut node, computed)) = tooltip.single_mut() {
            if let Ok(window) = window.single() {
                node.left = px((pos.x + TOOLTIP_OFFSET.x).clamp(0.0, window.width()-computed.size.x));
                node.top = px((pos.y + TOOLTIP_OFFSET.y).clamp(0.0, window.height()-computed.size.y));
            }
        }
    }
}

fn on_hover_show_highlight(
    items: Query<(&Hovered, &Children), (With<Interactable>, Changed<Hovered>)>,
    mut highlight: Query<&mut Visibility, With<Highlight>>,
) {
    for (hovered, children) in items {
        for child in children {
            if let Ok(mut visibility) = highlight.get_mut(*child) {
                *visibility = match hovered.get() {
                    false => Visibility::Hidden,
                    true => Visibility::Visible,
                }
            }
        }
    }
}

fn debug_items(
    items: Query<(Entity, &Sprite, &Transform, &Hovered), With<Interactable>>,
    tooltip: Query<(Entity, &Visibility, &Node, &Children), With<Tooltip>>,
    texts: Query<&Text>,
) {
    for (entity, sprite, transform, hovered) in &items {
        println!(
            "Item: {:?}, image: {:?}, position: {:?}, hovered: {:?}",
            entity, sprite.image, transform.translation, hovered
        );
    }
    println!("tooltips: {}", tooltip.iter().len());
    for (entity, visibility, node, children) in tooltip {
        for text_id in children {
            println!("{:?}", texts.get(*text_id).unwrap().0.clone());
        }
        println!(
            "Tooltip: {:?}, visibility: {:?}, position: {:?}, children: {:?}",
            entity,
            visibility,
            (node.top, node.left),
            children
        );
    }
}
