//! The credits menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{asset_tracking::LoadResource, audio::music, menus::Menu, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.load_resource::<CreditsAssets>();
    // app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_image = asset_server.load("images/gui/Button.png");
    let highlight = asset_server.load("images/gui/Button_Highlight.png");

    commands.spawn_scene(bsn! {
        @widget::ui_root("Credits Menu")
        GlobalZIndex(2)
        DespawnOnExit<Menu>(Menu::Credits)
        Children [
            @widget::header("Created by")
            --
            @created_by()
            --
            @widget::header("Assets")
            --
            @assets()
            --
            @widget::header("Voices")
            --
            @voices()
            --
            @widget::button("Back", go_back_on_click, button_image.clone(), highlight.clone())
        ]
    });
}

fn created_by() -> impl Scene {
    grid(vec![
        ["Magi3r", "Code"],
        ["Goldhand", "Art and Code"],
        ["Anabelle Kahns", "Sounds"],
        ["Palladium02", "Code"],
    ])
}

fn assets() -> impl Scene {
    grid(vec![
        ["Sound Effects", "by Anabelle Kahns or via Pixabay"],
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Images", "by Goldhand"],
        ["Font", "by MCKL via Google Fonts"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
    ])
}

fn voices() -> impl Scene {
    grid(vec![
        ["Wanderer", "Anabelle Kahns"],
        ["Kid", "Goldhand"],
        ["Wizard", "Benjamin von Freden"],
        ["King", ""],
        ["Rat", "Anabelle Kahns"]

    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Scene {
    let cols: Vec<RepeatedGridTrack> = RepeatedGridTrack::px(2, 400.0);

    let ch = content
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, text)| {
            let j = if i.is_multiple_of(2) {
                JustifySelf::End
            } else {
                JustifySelf::Start
            };

            Box::new(bsn! {
                @widget::label(text)
                Node {
                    justify_self: j,
                }
            })
        })
        .collect::<Vec<_>>();

    bsn! {
        #Grid
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: cols,
        }

        Children [ { ch } ]
    }
}

fn go_back_on_click(_: On<PointerClick>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn_scene(bsn! {
        #CreditsMusic
        DespawnOnExit<Menu>(Menu::Credits)
        @music(credits_music.music.clone())
    });
}
