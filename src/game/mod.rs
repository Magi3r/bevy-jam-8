use bevy::{
    ecs::template::OptionTemplate, image::TextureAtlasTemplate, picking::hover::Hovered,
    prelude::*, window::PrimaryWindow,
};

use crate::{
    audio::sound_effect,
    game::{
        animation::{CauldronAnimation, FireAnimation, MortarAnimation, StirerAnimation},
        level::{CauldronAmbienceSFX, FireAmbienceSFX, LevelAssets},
    },
    }, screens::{
        Screen::Gameplay, gameplay::{
            cooking_area::{Cauldron, Fire, Pull, Stirer}, storefront::Mortar,
    }, screens::{
        Screen::Gameplay, gameplay::{
            cooking_area::{Cauldron, Fire, FireState, Pull, Stirer}, storefront::Mortar,
        },
    },
    theme::palette::BUTTON_BACKGROUND,
};

const TOOLTIP_OFFSET: Vec2 = Vec2::new(15.0, 15.0);

pub mod alchemy;
pub mod animation;
pub mod level;
pub mod progression;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        alchemy::plugin,
        progression::plugin,
        level::plugin,
        animation::plugin,
    ));
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
    bsn! {
        Interactable {name: "Fire", description: "Burns stuff and heats cauldron"}
        Sprite {
            image:  { image },
        }
        Transform::from_xyz(0.0,0.0,0.5)
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
            height: px(150),
            width: px(600),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::top(px(40)),
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
        }
        // BackgroundColor(BUTTON_BACKGROUND)
        ImageNode {image:"images/gui/Popup_Highlight.png"}
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
                node.left =
                    px((pos.x + TOOLTIP_OFFSET.x).clamp(0.0, window.width() - computed.size.x));
                node.top =
                    px((pos.y + TOOLTIP_OFFSET.y).clamp(0.0, window.height() - computed.size.y));
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

pub fn on_click_cauldron_empty(
    _event: On<PointerClick>,
    cauldron: Query<Entity, (With<Cauldron>, Without<CauldronAnimation>)>,
    mut stirer: Query<&mut Visibility, With<Stirer>>,
    mut pull: Query<&mut Visibility, (With<Pull>, Without<Stirer>)>,
    level_assets: If<Res<LevelAssets>>,
    mut commands: Commands,
) {
    // do not start cauldron animation again if its already running
    if let Ok(cauldron) = cauldron.single() {
        commands
            .entity(cauldron)
            .insert_if_new(CauldronAnimation::new());
        *stirer.single_mut().unwrap() = Visibility::Hidden;
        *pull.single_mut().unwrap() = Visibility::Hidden;
        commands.spawn_scene(sound_effect(level_assets.cauldron_empty_sfx.clone()));
    }
}

pub fn on_click_stir_cauldron(
    _event: On<PointerClick>,
    stirer: Query<Entity, (With<Stirer>, Without<StirerAnimation>)>,
    cauldron_animation: Query<&CauldronAnimation>,
    level_assets: If<Res<LevelAssets>>,
    mut commands: Commands,
) {
    // do not start stiring if cauldron animation already running
    if !cauldron_animation.is_empty() {
        return;
    }
    if let Ok(stirer) = stirer.single() {
        commands
            .entity(stirer)
            .insert_if_new(StirerAnimation::new());
        commands.spawn_scene(sound_effect(level_assets.cauldron_stir_sfx.clone()));
    }
}

pub fn on_click_morter(
    _event: On<PointerClick>,
    mortar: Query<Entity, (With<Mortar>, Without<MortarAnimation>)>,
    level_assets: If<Res<LevelAssets>>,
    mut commands: Commands,
) {
    if let Ok(mortar) = mortar.single() {
        commands
            .entity(mortar)
            .insert_if_new(MortarAnimation::new());
        // commands.spawn_scene(sound_effect(level_assets.cauldron_stir_sfx.clone()));
    }
}

pub fn on_click_toggle_fire_on_off(
    _event: On<PointerClick>,
    mut fire: Query<(Entity,&mut Sprite), With<Fire>>,
    mut fire_state: ResMut<FireState>,
    mut commands: Commands,
    level_assets: If<Res<LevelAssets>>,
    fire_sfx: Query<&AudioSink, With<FireAmbienceSFX>>,
    // cauldron_sfx: Query<&AudioSink, (With<CauldronAmbienceSFX>, Without<FireAmbienceSFX>)>,
) {
    // do not start cauldron animation again if its already running
    if let Ok((entity, mut sprite)) = fire.single_mut() {
        if fire_state.0 {
            // toggle fire off
            fire_state.0 = false;
            sprite.image = level_assets.fire_off.clone();
            commands.entity(entity).remove::<FireAnimation>();
            if let Ok(sink) = fire_sfx.single() {
                sink.pause();
            }
            // if let Ok(sink) = cauldron_sfx.single() {
            // sink.pause();
            // }
        } else {
            // toggle fire on
            fire_state.0 = true;
            commands.entity(entity).insert(FireAnimation::new());
            if let Ok(sink) = fire_sfx.single() {
                sink.play();
            }
            // if let Ok(sink) = cauldron_sfx.single() {
            //     sink.play();
            // }
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
