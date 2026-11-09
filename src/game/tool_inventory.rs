use bevy::prelude::*;

use crate::game::alchemy::{AlchemyFluid, AlchemyItems, Element};

#[derive(Resource, Clone)]
pub struct CauldronInventory {
    pub contents: Vec<(ElementOrItem, usize)>,
}

impl Default for CauldronInventory {
    fn default() -> Self {
        Self {
            contents: vec![(ElementOrItem::Element(Element::Water), 10)],
        }
    }
}

#[derive(Resource, Clone)]
pub struct MortarInventory {
    pub contents: Vec<(ElementOrItem, usize)>,
}

impl Default for MortarInventory {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Resource, Clone)]
pub struct NPCInventory {
    pub contents: Vec<(ElementOrItem, usize)>,
}

impl Default for NPCInventory {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElementOrItem {
    Element(Element),
    Item(AlchemyItems),
    Fluid(AlchemyFluid),
}

impl ElementOrItem {
    pub fn image_path(&self) -> &'static str {
        match self {
            Self::Element(elem) => match elem {
                Element::Metal => "images/icons/Metal.png",
                Element::Life => "images/icons/Plant.png",
                Element::Water => "images/icons/Water.png",
                Element::Stone => "images/icons/Stone.png",
                Element::Fire => "images/icons/Fire.png",
                Element::Ice => "images/icons/Ice.png",
                Element::Soul => "images/icons/Soul.png",
            },
            Self::Item(item) => match item {
                AlchemyItems::JellyNettle => "images/items/JellyNettle.png",
                AlchemyItems::Sunflower => "images/items/Sunflower.png",
                AlchemyItems::IcyLilly => "images/items/IcyLilly.png",
                AlchemyItems::OrbClear => "images/items/OrbClear.png",
                AlchemyItems::OrbUnclear => "images/items/OrbUnclear.png",
                AlchemyItems::OrbBroken => "images/items/OrbBroken.png",
                AlchemyItems::JellyPowder => "images/items/JellyPowder.png",
                AlchemyItems::IcyPowder => "images/items/IcyPowder.png",
                AlchemyItems::SunflowerPowder => "images/items/SunPowder.png",
                AlchemyItems::HealthPotion => "images/items/Flask.png",
                AlchemyItems::BasicPotion => "images/items/Flask.png",
                AlchemyItems::MoltenSnowman => "images/items/MoltenSnowman.png",
                AlchemyItems::Snowman => "images/items/Snowman.png",
                AlchemyItems::EternalSnow => "images/items/EternalSnow.png",
                AlchemyItems::Flask => "images/items/Flask.png",
                AlchemyItems::PhilosopherStone => "images/items/PhilosopherStone.png",
                AlchemyItems::Lead => "images/items/Lead.png",
                AlchemyItems::Gold => "images/items/Gold.png",
                AlchemyItems::Cheese => "images/items/Cheese.png",
            },
            Self::Fluid(fluid) => match fluid {
                AlchemyFluid::BasicPotion => "assets/images/icons/Takeout.png",
                AlchemyFluid::HealthPotion => "assets/images/icons/Takeout.png",
            },
        }
    }

    /// Get a display name for this element or item.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Element(elem) => match elem {
                Element::Metal => "Metal",
                Element::Life => "Life",
                Element::Water => "Water",
                Element::Stone => "Stone",
                Element::Fire => "Fire",
                Element::Ice => "Ice",
                Element::Soul => "Soul",
            },
            Self::Item(item) => match item {
                AlchemyItems::JellyNettle => "Jelly Nettle",
                AlchemyItems::Sunflower => "Sunflower",
                AlchemyItems::IcyLilly => "Icy Lilly",
                AlchemyItems::OrbClear => "Orb of Pondering",
                AlchemyItems::OrbUnclear => "Dull Orb of Pondering",
                AlchemyItems::OrbBroken => "Shards of an Orb",
                AlchemyItems::JellyPowder => "Jelly Nettle Powder",
                AlchemyItems::IcyPowder => "Icy Nettle Powder",
                AlchemyItems::SunflowerPowder => "Sunflower Powder",
                AlchemyItems::HealthPotion => "Health Potion",
                AlchemyItems::BasicPotion => "Basic Potion",
                AlchemyItems::MoltenSnowman => "Molten Snowman",
                AlchemyItems::Snowman => "Unmelting Snowman",
                AlchemyItems::EternalSnow => "Unmelting Snow",
                AlchemyItems::Flask => "Flask",
                AlchemyItems::PhilosopherStone => "Philosophers Stone",
                AlchemyItems::Lead => "Lead",
                AlchemyItems::Gold => "Gold",
                AlchemyItems::Cheese => "Cheese",
            },
            Self::Fluid(fluid) => match fluid {
                AlchemyFluid::BasicPotion => "Basic Potion",
                AlchemyFluid::HealthPotion => "Health Potion",
            },
        }
    }
}
