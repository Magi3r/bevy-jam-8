//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::palette::TEMP_CHANGE_ME_SOON_COLOR};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (open_main_menu, show_backdrop));
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn show_backdrop(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_scene(bsn! {
        #Background
        Sprite {image: "images/gui/TitleScreen.png"}
        Transform::from_xyz(0.0,0.0,-1.0)
        DespawnOnExit<Screen>(Screen::Title)
    });
}
