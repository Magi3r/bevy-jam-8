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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElementOrItem {
    Element(Element),
    Ingredient(AlchemyItems),
    Fluid(AlchemyFluid),
}
