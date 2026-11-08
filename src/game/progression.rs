use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_inspector_egui::egui::TextStyle;
use serde::{Deserialize, Serialize};
use tracing::Level;

use crate::{
    asset_tracking::LoadResource,
    audio::{Voice, voice},
    game::{
        alchemy::{AlchemyEffect, AlchemyItems},
        level::LevelAssets,
    },
    screens::{
        Screen::Gameplay,
        gameplay::ShopArea::{self, Storefront},
    },
    theme::palette::BLACK,
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<Progression>();
    app.add_plugins(RonAssetPlugin::<Progression>::new(&["quests.ron"]));
    app.add_systems(OnEnter(Gameplay), debug_dump_progression);
    app.add_systems(OnEnter(Gameplay), init_assets);
    app.add_systems(OnEnter(Storefront), play_quest_start_voiceline);
    app.add_observer(show_textbox);
    app.add_observer(remove_textbox);
}

fn debug_dump_progression(
    assets: Res<AssetServer>,
    level_assets: If<Res<LevelAssets>>,
    progression: If<Res<Assets<Progression>>>,
) {
    let progression = progression.0.get(&level_assets.progression);
    println!("{:#?}", &progression);
}

fn init_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    level_assets: If<Res<LevelAssets>>,
    progression: If<Res<Assets<Progression>>>,
) {
    let quests = progression.get(&level_assets.progression).unwrap();
    commands.insert_resource(quests.clone());
    commands.insert_resource(QuestAssets::from_progression(quests, assets));
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub struct Quest {
    pub npc_name: String,
    pub npc_description: String,
    pub npc_asset: String,
    pub npc_voice_start: String,
    pub npc_text_start: String,
    pub npc_voice_finish: String,
    pub npc_text_finish: String,
    pub unlocks: Vec<AlchemyItems>,
    pub provides: Vec<AlchemyItems>,
    pub requests: NpcRequests,
    pub started: bool,
    pub finished: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Reflect)]
pub enum NpcRequests {
    Item(AlchemyItems),
    Effect(AlchemyEffect),
}

#[derive(Serialize, Deserialize, Asset, Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct Progression {
    pub quests: Vec<Quest>,

    pub stage: usize,
}

#[derive(Component, Default, Clone)]
pub struct Npc;

#[derive(Resource, Asset, Clone, Reflect, Default)]
#[reflect(Resource)]
pub struct QuestAssets {
    #[dependency]
    pub npc_assets: Vec<Handle<Image>>,
    #[dependency]
    pub npc_voice_start: Vec<Handle<AudioSource>>,
    #[dependency]
    pub npc_voice_finsh: Vec<Handle<AudioSource>>,
}

impl QuestAssets {
    pub fn from_progression(progression: &Progression, asset_server: Res<AssetServer>) -> Self {
        let mut npc_assets = vec![];
        let mut npc_voice_start = vec![];
        let mut npc_voice_finsh = vec![];
        for quest in &progression.quests {
            npc_assets.push(asset_server.load(quest.npc_asset.clone()));
            npc_voice_start.push(asset_server.load(quest.npc_voice_start.clone()));
            npc_voice_finsh.push(asset_server.load(quest.npc_voice_finish.clone()));
        }
        QuestAssets {
            npc_assets,
            npc_voice_start,
            npc_voice_finsh,
        }
    }
}

fn play_quest_start_voiceline(
    mut commands: Commands,
    quest_assets: Res<QuestAssets>,
    mut progression_assets: ResMut<Progression>,
) {
    let stage = progression_assets.stage;
    if !progression_assets.quests.get(stage).unwrap().started {
        progression_assets.quests.get_mut(stage).unwrap().started = true;
        commands.spawn_scene(voice(
            quest_assets.npc_voice_start.get(stage).unwrap().clone(),
        ));
    }
}

pub fn on_click_play_quest_start_voiceline(
    _: On<PointerClick>,
    mut commands: Commands,
    quest_assets: Res<QuestAssets>,
    progression_assets: ResMut<Progression>,
    voice_q: Query<&Voice>,
) {
    if voice_q.is_empty() {
        let stage = progression_assets.stage;
        commands.spawn_scene(voice(
            quest_assets.npc_voice_start.get(stage).unwrap().clone(),
        ));
    }
}

fn show_textbox(
    _: On<Insert<Voice>>,
    mut commands: Commands,
    progression_assets: Res<Progression>,
    assets: Res<AssetServer>,
) {
    let stage = progression_assets.stage;
    let quest = progression_assets.quests.get(stage).unwrap();
    let text;
    if !quest.finished {
        text = &quest.npc_text_start;
    } else {
        text = &quest.npc_text_finish;
    }
    commands.spawn_scene(textbox(assets, text));
}

fn remove_textbox(
    _: On<Remove<Voice>>,
    mut commands: Commands,
    textbox: Query<Entity, With<Textbox>>,
) {
    for entity in textbox {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
pub struct Textbox;

fn textbox(assets: Res<AssetServer>, text: &String) -> impl Scene {
    bsn! {
        Textbox
        ImageNode {image: {assets.load("images/gui/Textbox_Highlight.png")}, image_mode: NodeImageMode::Stretch, }
        Node {
            position_type: PositionType::Absolute,
            bottom: px(40),
            width: percent(100)
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center
        }
        Children [
            Text(text)
            TextColor(BLACK)
            TextFont {font_size: px(40.)}
            Node {max_width: percent(90), align_items: AlignItems::Center, padding: UiRect { left: px(40), right: px(40), top: px(80), bottom: px(80) }}
        ]
    }
}
