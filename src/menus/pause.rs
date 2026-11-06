//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    screens::{
        Screen,
        gameplay::ShopArea::{self, TransitionAnimation},
    },
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_image = asset_server.load("images/gui/Button.png");
    let highlight = asset_server.load("images/gui/Button_Highlight.png");

    commands.spawn_scene(bsn! {
        @widget::ui_root("Pause Menu")
        GlobalZIndex(2)
        DespawnOnExit<Menu>(Menu::Pause)
        Children [
            @widget::header("Game paused")
            --
            @widget::button("Continue", close_menu, button_image.clone(), highlight.clone())
            --
            @widget::button("Settings", open_settings_menu, button_image.clone(), highlight.clone())
            --
            @widget::button("Quit to title", quit_to_title, button_image.clone(), highlight.clone())
        ]
    });
}

fn open_settings_menu(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(
    _: On<PointerClick>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_shoparea: ResMut<NextState<ShopArea>>,
) {
    next_screen.set(Screen::Title);
    next_shoparea.set(TransitionAnimation);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
