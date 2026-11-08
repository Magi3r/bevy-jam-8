use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::level::LevelAssets;

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<RecipeBook>();
    app.add_plugins(RonAssetPlugin::<RecipeBook>::new(&["recipes.ron"]));

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
    name: String,
    ingredients: Vec<(Ingredient, usize)>,
    produces: Vec<(Ingredient, usize)>,
    uses: Tool,
    discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ingredient {
    Alchemy(AlchemyItems),
    Element(Element),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub enum AlchemyItems {
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
    MoltenSnowman,
    Snowman,
    Flask,
    PhilosopherStone,
    Lead,
    Gold,
    Cheese,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum AlchemyEffect {
    Explosion,
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
            produces: vec![(Ingredient::Alchemy(AlchemyItems::NettleDust), 1)],
            uses: Tool::Mortar,
            discovered: true,
        };

        let recipe_book = RecipeBook(vec![_1, _2]);
        fs::write("test.ron", ron::to_string(&recipe_book).unwrap());
    }
}
