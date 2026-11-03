//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::WindowResolution};

use crate::{Pause, game::level::spawn_level, menus::Menu, screens::Screen};

pub mod cooking_area;
pub mod storefront;

const CAM_PAN_SPEED: f32 = 100.0;
const CAM_MIN_X: f32 = -1000.0;
const CAM_MAX_X: f32 = 1000.0;



/// The game's in-game areas.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ShopArea {
    #[default]
    Storefront,
    CookingArea,
}

impl ShopArea {
    fn get_area_left(&self) -> Option<ShopArea> {
        match self {
            ShopArea::Storefront => None,
            ShopArea::CookingArea => Some(ShopArea::Storefront),
        }
    }

    fn get_area_right(&self) -> Option<ShopArea> {
        match self {
            ShopArea::Storefront => Some(ShopArea::CookingArea),
            ShopArea::CookingArea => None,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.init_state::<ShopArea>();
    app.add_plugins((cooking_area::plugin, storefront::plugin));
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and_then(in_state(Menu::None))
                    .and_then(
                        input_just_pressed(KeyCode::KeyP)
                            .or_else(input_just_pressed(KeyCode::Escape)),
                    ),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and_then(not(in_state(Menu::None)))
                    .and_then(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, edge_pan_camera.run_if(in_state(Screen::Gameplay)));
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #PauseOverlay
        Node {
            width: percent(100),
            height: percent(100),
        }
        GlobalZIndex(1)
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8))
        DespawnOnExit<Pause>(Pause(true))
    });
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn edge_pan_camera(
    window: Query<&Window>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut next_shop_area: ResMut<NextState<ShopArea>>,
    current_shop_area: Res<State<ShopArea>>,
) {
    let window_width = window.single().unwrap().width();
    let mouse = window.single().unwrap().cursor_position();
    let edge_size = window_width * 0.02; // 2% window width for cam panning
    let mut cam_pos = camera.single_mut().unwrap();
    if let Some(mouse_pos) = mouse {
        // println!("window_width: {}, mouse_pos.x: {}, edge_size: {}, cam_pos.x: {}", window_width, mouse_pos.x, edge_size, cam_pos.translation.x);
        if mouse_pos.x < edge_size + cam_pos.translation.x {
            if let Some(area_left) = current_shop_area.get_area_left() {
                // TODO: play animation
                next_shop_area.set(area_left);
            }
            //     // pan left
            //     if cam_pos.translation.x > CAM_MIN_X {
            //         cam_pos.translation.x -= CAM_PAN_SPEED;
            //         cam_pos.translation.x = cam_pos.translation.x.clamp(CAM_MIN_X, CAM_MAX_X);
            //     }
        }
        if mouse_pos.x > window_width - edge_size + cam_pos.translation.x {
            if let Some(area_right) = current_shop_area.get_area_right() {
                // TODO: play animation
                next_shop_area.set(area_right);
            }
            //     // pan right
            //     if cam_pos.translation.x > CAM_MIN_X {
            //         cam_pos.translation.x += CAM_PAN_SPEED;
            //         cam_pos.translation.x = cam_pos.translation.x.clamp(CAM_MIN_X, CAM_MAX_X);
            //     }
        }
    }
}
