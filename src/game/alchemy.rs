use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    game::{
        level::LevelAssets, progression::Npc, tool_inventory::{CauldronInventory, ElementOrItem, MortarInventory, NPCInventory},
    }, screens::{
        Screen::Gameplay,
        gameplay::{
            cooking_area::{Cauldron, Fire, FireState},
            storefront::Mortar,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<RecipeBook>();
    app.init_resource::<HeldItem>();
    app.add_plugins(RonAssetPlugin::<RecipeBook>::new(&["recipes.ron"]));
    app.add_observer(on_click_do_something_with_item);
    app.add_systems(Update, update_held_item_pos.run_if(in_state(Gameplay)));
    // #[cfg(feature = "dev")]
    // app.add_systems(Update, debug_dump_recipebook);
    // app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    // app.add_systems(
    //     Update,
    //     record_player_directional_input
    //         .in_set(AppSystems::RecordInput)
    //         .in_set(PausableSystems),
    // );
}

fn debug_dump_recipebook(level_assets: If<Res<LevelAssets>>, recipes: If<Res<Assets<RecipeBook>>>) {
    let recipes = recipes.0.get(&level_assets.0.recipes);
    println!("{:?}", &recipes);
}

#[derive(Asset, TypePath, Serialize, Deserialize, Debug)]
pub struct RecipeBook(pub Vec<Recipe>);

/// Index structure to see whether a combination of ingredients with the quantities is a registered recipe.
pub struct RecipeIndex(pub HashMap<Vec<(Ingredient, usize)>, Recipe>);

impl RecipeIndex {
    pub fn from_recipes(recipes: &[Recipe]) -> Self {
        let index = recipes
            .iter()
            .map(|recipe| {
                let mut ingredients = recipe.ingredients.clone();
                ingredients.sort_by_key(|(ingredient, _)| ingredient.clone());

                (ingredients, recipe.clone())
            })
            .collect();

        Self(index)
    }

    pub fn recipe_exists(
        &self,
        ingredients: &mut [(Ingredient, usize)],
        came_from: Tool,
    ) -> Option<&Recipe> {
        ingredients.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        match self.0.get(ingredients) {
            Some(recipe) if recipe.uses == came_from => Some(recipe),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<(Ingredient, usize)>,
    pub produces: Vec<(Ingredient, usize)>,
    pub uses: Tool,
    pub discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ingredient {
    Alchemy(AlchemyItems),
    Fluid(AlchemyFluid),
    Element(Element),
}

impl Ingredient {
    pub fn as_element_or_item(&self) -> ElementOrItem {
        match self {
            Self::Alchemy(items) => ElementOrItem::Item(items.clone()),
            Self::Fluid(fluid) => ElementOrItem::Fluid(fluid.clone()),
            Self::Element(elem) => ElementOrItem::Element(elem.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Default)]
pub enum Element {
    #[default]
    Metal,
    Life,
    Water,
    Stone,
    Fire,
    Ice,
    Soul,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Tool {
    Mortar,
    Cauldron,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Default,
)]
pub enum AlchemyItems {
    #[default]
    JellyNettle,
    Sunflower,
    IcyLilly,
    OrbClear,
    OrbUnclear,
    OrbBroken,
    JellyPowder,
    IcyPowder,
    SunflowerPowder,
    HealthPotion,
    BasicPotion,
    MoltenSnowman,
    Snowman,
    EternalSnow,
    Flask,
    PhilosopherStone,
    Lead,
    Gold,
    Cheese,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub enum AlchemyFluid {
    BasicPotion,
    HealthPotion,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum AlchemyEffect {
    Explosion,
}

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
pub struct Draggable;

#[derive(Resource, Reflect, Debug, Clone, Default)]
#[reflect(Resource)]
pub struct HeldItem {
    pub entity: Option<Entity>,
    pub item: Option<AlchemyItems>,
}

#[derive(Component, Reflect, Debug, Default, Clone)]
#[reflect(Component)]
pub struct GetOnClick(pub AlchemyItems, pub String);

fn on_click_do_something_with_item(
    event: On<PointerClick>,
    mut commands: Commands,
    mut held_item: ResMut<HeldItem>,
    get_on_click: Query<&GetOnClick>,
    fire: Query<&Fire>,
    fire_state: Res<FireState>,
    mortar: Query<&Mortar>,
    mut mortar_inventory: ResMut<MortarInventory>,
    cauldron: Query<&Cauldron>,
    mut cauldron_inventory: ResMut<CauldronInventory>,
    npc: Query<&Npc>,
    mut npc_inventory: ResMut<NPCInventory>,
) {
    // get item
    if held_item.entity.is_none() {
        if let Ok(clicked_entity_item) = get_on_click.get(event.entity) {
            let id = commands
                .spawn_scene(bsn! {
                    Draggable
                    Sprite {image: {clicked_entity_item.1.clone()} }
                    Transform::from_xyz(0.,0.,50.).with_scale((0.2,0.2,1.0).into())
                })
                .id();
            held_item.entity = Some(id);
            held_item.item = Some(clicked_entity_item.0.clone())
        }
    } else {
        // put item
        if let Some(held_entity) = held_item.entity.clone()
            && let Some(held_item_item) = &mut held_item.item.clone()
        {
            let entity = event.entity;
            // destroy if click on fire
            if fire.get(entity).is_ok() && fire_state.0 {
                debug!("Fire!");
                commands.entity(held_entity).despawn();
                held_item.entity = None;
                held_item.item = None;
            }
            // destroy if click on same item
            if let Ok(inventory) = get_on_click.get(entity)
                && inventory.0 == *held_item_item
            {
                debug!("Same Item!");
                commands.entity(held_entity).despawn();
                held_item.entity = None;
                held_item.item = None;
            }
            // do something if on mortar
            if mortar.get(event.entity).is_ok() {
                debug!("Mortar!");
                commands.entity(held_entity).despawn();
                held_item.entity = None;
                held_item.item = None;
                if let Some(item) = mortar_inventory
                    .contents
                    .iter_mut()
                    .find(|item| item.0 == ElementOrItem::Item(held_item_item.clone()))
                {
                    item.1 += 1
                } else {
                    mortar_inventory
                        .contents
                        .push((ElementOrItem::Item(held_item_item.clone()), 1));
                }
            }
            // do something if on cauldron
            if cauldron.get(event.entity).is_ok() {
                debug!("Cauldron!");
                commands.entity(held_entity).despawn();
                held_item.entity = None;
                held_item.item = None;
                if let Some(item) = cauldron_inventory
                    .contents
                    .iter_mut()
                    .find(|item| item.0 == ElementOrItem::Item(held_item_item.clone()))
                {
                    item.1 += 1
                } else {
                    cauldron_inventory
                        .contents
                        .push((ElementOrItem::Item(held_item_item.clone()), 1));
                }
            }
            // do something if on npc
            if npc.get(event.entity).is_ok() {
                debug!("NPC!");
                commands.entity(held_entity).despawn();
                held_item.entity = None;
                held_item.item = None;
                if let Some(item) = npc_inventory
                    .contents
                    .iter_mut()
                    .find(|item| item.0 == ElementOrItem::Item(held_item_item.clone()))
                {
                    item.1 += 1
                } else {
                    npc_inventory
                        .contents
                        .push((ElementOrItem::Item(held_item_item.clone()), 1));
                }
            }
        }
    }
}

fn update_held_item_pos(
    item: Query<&mut Transform, With<Draggable>>,
    window: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if let Ok(window) = window.single() {
        if let Some(pos) = window.cursor_position() {
            if let Ok(world_position) = camera.0.viewport_to_world_2d(camera.1, pos) {
                for mut transfrom in item {
                    transfrom.translation.x = world_position.x;
                    transfrom.translation.y = world_position.y;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::fs;

    use bevy_common_assets::ron::RonAssetPlugin;

    use super::*;

    #[test]
    fn test() {
        let _1 = Recipe {
            name: "Health Potion".into(),
            ingredients: vec![
                (Ingredient::Element(Element::Life), 1),
                (Ingredient::Alchemy(AlchemyItems::JellyNettle), 1),
            ],
            produces: vec![(Ingredient::Alchemy(AlchemyItems::HealthPotion), 1)],
            uses: Tool::Cauldron,
            discovered: true,
        };

        let _2 = Recipe {
            name: "Life".into(),
            ingredients: vec![(Ingredient::Alchemy(AlchemyItems::JellyNettle), 1)],
            produces: vec![(Ingredient::Alchemy(AlchemyItems::JellyNettle), 1)],
            uses: Tool::Mortar,
            discovered: true,
        };

        let recipe_book = RecipeBook(vec![_1, _2]);
        fs::write("test.ron", ron::to_string(&recipe_book).unwrap());
    }
}
