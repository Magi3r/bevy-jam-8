use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    // app.add_systems(
    //     Update,
    //     record_player_directional_input
    //         .in_set(AppSystems::RecordInput)
    //         .in_set(PausableSystems),
    // );
}

struct RecipeBook(Vec<Recipe>);

struct Recipe {
    ingredient: Option<Ingredient>,
    elements: Option<Vec<Element>>,
    result: RecipeResult,
    recipe_type: RecipeType,
    discovered: bool,
}

enum Element {
    Plant,
    Mana,
    Water,
    Life,
}

enum RecipeResult {
    Ingredient(Ingredient),
    Element(Element),
}

enum RecipeType {
    Grinding,
    PutInCauldron,
}

struct Ingredient {
    name: String,
    description: String,
    cauldron_effect: CauldronEffect,
}

enum CauldronEffect {
    RemoveElement(Element),
    TransformElement { from: Element, to: Element },
}
