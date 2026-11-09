//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use std::time::Duration;
use tracing::Level;

use crate::{
    AppSystems, PausableSystems,
    game::level::LevelAssets,
    screens::{
        Screen::Gameplay,
        gameplay::{
            ShopArea::{self, CookingArea, Storefront},
            cooking_area::{Pull, Stirer},
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            (
                update_fire_animation_timer,
                update_transition_animation_timer,
                update_cauldron_empty_animation_timer,
                update_stirer_animation_timer,
                update_mortar_animation_timer,
                update_explosion_timer,
            )
                .in_set(AppSystems::TickTimers),
            (
                update_explosion_sprite.run_if(in_state(ShopArea::CookingArea)),
                update_fire_animation_sprite.run_if(in_state(ShopArea::CookingArea)),
                update_transition_animation_sprite.run_if(in_state(ShopArea::TransitionAnimation)),
                next_state_when_transition_finished.run_if(in_state(ShopArea::TransitionAnimation)),
                (
                    update_cauldron_empty_animation_sprite,
                    reverse_anim_when_cauldron_empty_finished,
                )
                    .chain()
                    .run_if(in_state(ShopArea::CookingArea)),
                (
                    update_stirer_animation_sprite,
                    remove_component_when_animation_finished,
                )
                    .chain()
                    .run_if(in_state(ShopArea::CookingArea)),
                (
                    update_mortar_animation_sprite,
                    remove_component_when_animation_finished_mortar,
                )
                    .chain()
                    .run_if(in_state(ShopArea::Storefront)),
            )
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the animation timer.
fn update_fire_animation_timer(time: Res<Time>, mut query: Query<&mut FireAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Component that tracks fire's animation state.
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct FireAnimation {
    timer: Timer,
    frame: usize,
}

impl FireAnimation {
    pub const ANIMATION_FRAMES: usize = 12;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(120);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1) % Self::ANIMATION_FRAMES;
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_fire_animation_sprite(
    mut query: Query<(&FireAnimation, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets.fire.get(animation.frame).unwrap().clone();
        }
    }
}

/// Update the animation timer.
fn update_explosion_timer(time: Res<Time>, mut query: Query<&mut Explostion>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Component that tracks fire's animation state.
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Explostion {
    timer: Timer,
    frame: usize,
}

impl Explostion {
    pub const ANIMATION_FRAMES: usize = 48;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(42);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1) % Self::ANIMATION_FRAMES;
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_explosion_sprite(
    mut query: Query<(&Explostion, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets.fire.get(animation.frame).unwrap().clone();
        }
    }
}

pub fn transition_scene(
    left_to_right: bool,
    next_state: ShopArea,
    level_assets: &LevelAssets,
) -> impl Scene {
    let transition = TransitionAnimation::new(left_to_right, next_state);
    bsn! {
        #TransitionAnimation
        transition
        DespawnOnExit<ShopArea>(ShopArea::TransitionAnimation)
        Sprite {image: {level_assets.transition_animation.get(transition.frame).unwrap().clone()}}
    }
}

/// Update the animation timer.
fn update_transition_animation_timer(time: Res<Time>, mut query: Query<&mut TransitionAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}
/// Component that tracks fire's animation state.
#[derive(Component, Default, Clone)]
pub struct TransitionAnimation {
    timer: Timer,
    frame: usize,
    left_to_right: bool,
    next_state: ShopArea,
}

fn next_state_when_transition_finished(
    transitions: Query<&TransitionAnimation>,
    mut next_state: ResMut<NextState<ShopArea>>,
) {
    for transition in transitions {
        if transition.is_finished() {
            next_state.set(transition.next_state)
        }
    }
}

impl TransitionAnimation {
    pub const ANIMATION_FRAMES: usize = 12;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(42);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        if self.left_to_right && self.frame < Self::ANIMATION_FRAMES {
            self.frame += 1;
        } else if self.frame > 0 {
            self.frame -= 1;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn is_finished(&self) -> bool {
        if self.left_to_right {
            self.frame >= Self::ANIMATION_FRAMES
        } else {
            self.frame == 0
        }
    }

    pub(crate) fn new(left_to_right: bool, next_state: ShopArea) -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: if left_to_right {
                0
            } else {
                Self::ANIMATION_FRAMES - 1
            },
            left_to_right,
            next_state,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_transition_animation_sprite(
    mut query: Query<(&TransitionAnimation, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets
                .transition_animation
                .get(
                    animation
                        .frame
                        .clamp(0, TransitionAnimation::ANIMATION_FRAMES - 1),
                )
                .unwrap()
                .clone();
        }
    }
}

/// Update the animation timer.
fn update_cauldron_empty_animation_timer(
    time: Res<Time>,
    mut query: Query<&mut CauldronAnimation>,
) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}
/// Component that tracks fire's animation state.
#[derive(Component, Default, Clone)]
pub struct CauldronAnimation {
    timer: Timer,
    frame: usize,
    left_to_right: bool,
}

fn reverse_anim_when_cauldron_empty_finished(
    animations: Query<(Entity, &mut CauldronAnimation, &mut Sprite)>,
    mut stirer: Query<&mut Visibility, With<Stirer>>,
    mut pull: Query<&mut Visibility, (With<Pull>, Without<Stirer>)>,
    level_assets: Res<LevelAssets>,
    mut commands: Commands,
) {
    for (entity, mut anim, mut sprite) in animations {
        if anim.is_finished() {
            if anim.left_to_right {
                anim.left_to_right = false;
            } else {
                *stirer.single_mut().unwrap() = Visibility::Visible;
                *pull.single_mut().unwrap() = Visibility::Visible;
                commands.entity(entity).remove::<CauldronAnimation>();
                sprite.image = level_assets.cauldron.clone();
            }
        }
    }
}

impl CauldronAnimation {
    pub const ANIMATION_FRAMES: usize = 24;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(42);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        if self.left_to_right && self.frame < Self::ANIMATION_FRAMES {
            self.frame += 1;
        } else if self.frame > 0 {
            self.frame -= 1;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn is_finished(&self) -> bool {
        if self.left_to_right {
            self.frame >= Self::ANIMATION_FRAMES
        } else {
            self.frame == 0
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
            left_to_right: true,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_cauldron_empty_animation_sprite(
    mut query: Query<(&CauldronAnimation, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets
                .cauldron_empty
                .get(
                    animation
                        .frame
                        .clamp(0, CauldronAnimation::ANIMATION_FRAMES - 1),
                )
                .unwrap()
                .clone();
        }
    }
}

/// Update the animation timer.
fn update_stirer_animation_timer(time: Res<Time>, mut query: Query<&mut StirerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}
/// Component that tracks fire's animation state.
#[derive(Component, Default, Clone)]
pub struct StirerAnimation {
    timer: Timer,
    frame: usize,
}

impl StirerAnimation {
    pub const ANIMATION_FRAMES: usize = 24;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(42);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        if self.frame < Self::ANIMATION_FRAMES {
            self.frame += 1;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn is_finished(&self) -> bool {
        self.frame >= Self::ANIMATION_FRAMES
    }

    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_stirer_animation_sprite(
    mut query: Query<(&StirerAnimation, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets
                .stirer
                .get(
                    animation
                        .frame
                        .clamp(0, StirerAnimation::ANIMATION_FRAMES - 1),
                )
                .unwrap()
                .clone();
        }
    }
}

fn remove_component_when_animation_finished(
    transitions: Query<(Entity, &StirerAnimation)>,
    mut commands: Commands,
) {
    for (entity, anim) in transitions {
        if anim.is_finished() {
            commands.entity(entity).remove::<StirerAnimation>();
        }
    }
}

/// Update the animation timer.
fn update_mortar_animation_timer(time: Res<Time>, mut query: Query<&mut MortarAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}
/// Component that tracks fire's animation state.
#[derive(Component, Default, Clone)]
pub struct MortarAnimation {
    timer: Timer,
    frame: usize,
}

impl MortarAnimation {
    pub const ANIMATION_FRAMES: usize = 24;
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(42);
    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        if self.frame < Self::ANIMATION_FRAMES {
            self.frame += 1;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn is_finished(&self) -> bool {
        self.frame >= Self::ANIMATION_FRAMES
    }

    pub(crate) fn new() -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame: 0,
        }
    }
}

/// Update the texture to reflect changes in the animation.
fn update_mortar_animation_sprite(
    mut query: Query<(&MortarAnimation, &mut Sprite)>,
    level_assets: If<Res<LevelAssets>>,
) {
    for (animation, mut sprite) in &mut query {
        if animation.changed() {
            sprite.image = level_assets
                .mortar
                .get(
                    animation
                        .frame
                        .clamp(0, MortarAnimation::ANIMATION_FRAMES - 1),
                )
                .unwrap()
                .clone();
        }
    }
}

fn remove_component_when_animation_finished_mortar(
    transitions: Query<(Entity, &MortarAnimation)>,
    mut commands: Commands,
) {
    for (entity, anim) in transitions {
        if anim.is_finished() {
            commands.entity(entity).remove::<MortarAnimation>();
        }
    }
}

// /// If the player is moving, play a step sound effect synchronized with the
// /// animation.
// fn trigger_step_sound_effect(
//     mut commands: Commands,
//     player_assets: If<Res<PlayerAssets>>,
//     mut step_query: Query<&PlayerAnimation>,
// ) {
//     for animation in &mut step_query {
//         if animation.state == PlayerAnimationState::Walking
//             && animation.changed()
//             && (animation.frame == 2 || animation.frame == 5)
//         {
//             let rng = &mut rand::rng();
//             let random_step = player_assets.steps.choose(rng).unwrap().clone();
//             commands.spawn_scene(sound_effect(random_step));
//         }
//     }
// }
//
// /// Component that tracks player's animation state.
// /// It is tightly bound to the texture atlas we use.
// #[derive(Component, Reflect, Default, Clone)]
// #[reflect(Component)]
// pub struct PlayerAnimation {
//     timer: Timer,
//     frame: usize,
//     state: PlayerAnimationState,
// }

// #[derive(Reflect, PartialEq, Default, Clone)]
// pub enum PlayerAnimationState {
//     #[default]
//     Idling,
//     Walking,
// }

// impl PlayerAnimation {
//     /// The number of idle frames.
//     const IDLE_FRAMES: usize = 2;
//     /// The duration of each idle frame.
//     const IDLE_INTERVAL: Duration = Duration::from_millis(500);
//     /// The number of walking frames.
//     const WALKING_FRAMES: usize = 6;
//     /// The duration of each walking frame.
//     const WALKING_INTERVAL: Duration = Duration::from_millis(50);

//     fn idling() -> Self {
//         Self {
//             timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
//             frame: 0,
//             state: PlayerAnimationState::Idling,
//         }
//     }

//     fn walking() -> Self {
//         Self {
//             timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
//             frame: 0,
//             state: PlayerAnimationState::Walking,
//         }
//     }

//     pub fn new() -> Self {
//         Self::idling()
//     }

//     /// Update animation timers.
//     pub fn update_timer(&mut self, delta: Duration) {
//         self.timer.tick(delta);
//         if !self.timer.is_finished() {
//             return;
//         }
//         self.frame = (self.frame + 1)
//             % match self.state {
//                 PlayerAnimationState::Idling => Self::IDLE_FRAMES,
//                 PlayerAnimationState::Walking => Self::WALKING_FRAMES,
//             };
//     }

//     /// Update animation state if it changes.
//     pub fn update_state(&mut self, state: PlayerAnimationState) {
//         if self.state != state {
//             match state {
//                 PlayerAnimationState::Idling => *self = Self::idling(),
//                 PlayerAnimationState::Walking => *self = Self::walking(),
//             }
//         }
//     }

//     /// Whether animation changed this tick.
//     pub fn changed(&self) -> bool {
//         self.timer.is_finished()
//     }

//     /// Return sprite index in the atlas.
//     pub fn get_atlas_index(&self) -> usize {
//         match self.state {
//             PlayerAnimationState::Idling => self.frame,
//             PlayerAnimationState::Walking => 6 + self.frame,
//         }
//     }
// }
