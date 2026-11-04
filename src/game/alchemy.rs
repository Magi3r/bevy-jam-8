use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::game::level::LevelAssets;

pub(super) fn plugin(app: &mut App) {
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

fn debug_dump_recipebook(level_assets: If<Res<LevelAssets>>,
            recipes: If<Res<Assets<RecipeBook>>>) {
    let recipes = recipes.0.get(&level_assets.0.recipes);
    println!("{:?}", &recipes);
}

#[derive(Asset, TypePath, Serialize, Deserialize, Debug)]
pub struct RecipeBook(pub Vec<Recipe>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    name: String,
    ingredients: Vec<(Ingredient, usize)>,
    produces: Vec<(Ingredient, usize)>,
    uses: Tool,
    discovered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Ingredient {
    Alchemy(AlchemyItems),
    Element(Element),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Element {
    Metal,
    Life,
    Water,
    Stone,
    Fire,
    Ice,
    Soul,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Tool {
    Mortar,
    Cauldron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AlchemyItems {
    JellyNettle,
    NettleDust,
    HealthPotion,
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
