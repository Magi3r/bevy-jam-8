//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{ecs::system::IntoObserverSystem, prelude::*};
use bevy_inspector_egui::egui::epaint::text::FontPriority::Highest;

use crate::theme::{interaction::InteractionPalette, palette::*};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Scene {
    let name = name.into();
    bsn! {
        Name::new(name)
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
        }
        // Don't block picking events for other UI roots.
        Pickable::IGNORE
    }
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Header
        Text({ text.into() })
        TextFont { font_size: px(40.0) }
        TextColor(HEADER_TEXT)
    }
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Label
        Text({ text.into() })
        TextFont { font_size: px(26.0) }
        TextColor(LABEL_TEXT)
    }
}

pub fn title_label(text: impl Into<String>) -> impl Scene {
    bsn! {
        #Label
        Text({ text.into() })
        TextFont { font_size: px(60.0) }
        TextColor(LABEL_TEXT)
    }
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, M, I>(
    text: impl Into<String>,
    action: I,
    image: Handle<Image>,
    highlight: Handle<Image>,
) -> impl Scene
where
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    button_base(
        text,
        action,
        bsn! {
            Node {
                width: px(1920),
                height: px(175),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
        },
        image,
        highlight,
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, M, I>(
    text: impl Into<String>,
    action: I,
    image: Handle<Image>,
    highlight: Handle<Image>,
) -> impl Scene
where
    E: EntityEvent,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    button_base(
        text,
        action,
        bsn! {
            Node {
                width: px(125),
                height: px(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
        },
        image,
        highlight,
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Scene,
    image: Handle<Image>,
    highlight: Handle<Image>,
) -> impl Scene
where
    E: EventPattern<Event: EntityEvent>,
    I: IntoObserverSystem<E, M, ()> + Clone + Sync,
    M: 'static,
{
    let text = text.into();
    let pressed = highlight.clone();
    let image_node = image.clone();

    bsn! {
        #Button
        Node
        Children [
            #ButtonInner
            // Button
            // BackgroundColor(BUTTON_BACKGROUND)

            ImageNode {image : image_node}

            InteractionPalette {
                none: image,
                hovered: highlight,
                pressed: pressed,
            }

            Children [
                #ButtonText
                Text(text)
                TextFont { font_size: px(28.0) }
                TextColor(BUTTON_TEXT)
                // Don't bubble picking events from the text up to the button.
                Pickable::IGNORE
            ]
            @button_bundle
            @on(action)
        ]
    }
}
