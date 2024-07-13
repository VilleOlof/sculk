use simdnbt::Mutf8Str;
use std::borrow::Cow;

use crate::{error::SculkParseError, traits::FromCompoundNbt, util::get_bool};

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeBook<'a> {
    /// A list of all recipes the player has seen.
    pub recipes: Vec<Cow<'a, Mutf8Str>>,

    /// A list of all recipes the player has unlocked, but not viewed in the crafting helper yet.  
    /// `toBeDisplayed`
    pub to_be_displayed: Vec<Cow<'a, Mutf8Str>>,

    /// True if the player has enabled the "Show Craftable" feature in the crafting recipe book.  
    /// `isFilteringCraftable`
    pub is_filtering_craftable: bool,

    /// True if the player has the crafting recipe book GUI open.  
    /// `isGuiOpen`
    pub is_gui_open: bool,

    /// True if the player has enabled the "Show Smeltable" feature in the smelting recipe book.
    /// `isFurnaceFilteringCraftable`
    pub is_furnace_filtering_craftable: bool,

    /// True if the player has the smelting recipe book GUI open.
    /// `isFurnaceGuiOpen`
    pub is_furnace_gui_open: bool,

    /// True if the player has enabled the "Show Blastable" feature in the blasting recipe book.
    /// `isBlastingFurnaceFilteringCraftable`
    pub is_blasting_furnace_filtering_craftable: bool,

    /// True if the player has the blasting recipe book GUI open.
    /// `isBlastingFurnaceGuiOpen`
    pub is_blast_furnace_gui_open: bool,

    /// True if the player has enabled the "Show Smokable" feature in the smoking recipe book.
    /// `isSmokerFilteringCraftable`
    pub is_smoker_filtering_craftable: bool,

    /// True if the player has the smoking recipe book GUI open.
    /// `isSmokerGuiOpen`
    pub is_smoker_gui_open: bool,
}

impl<'a> FromCompoundNbt for RecipeBook<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let recipes = if let Some(recipes) = nbt.list("recipes") {
            let list = recipes
                .strings()
                .ok_or(SculkParseError::InvalidField("recipes".into()))?;

            let mut recipes: Vec<Cow<'a, Mutf8Str>> = vec![];

            for recipe in list {
                let str = (*recipe).to_owned();
                recipes.push(Cow::Owned(str));
            }

            recipes
        } else {
            vec![]
        };

        let to_be_displayed = if let Some(to_be_displayed) = nbt.list("toBeDisplayed") {
            let list = to_be_displayed
                .strings()
                .ok_or(SculkParseError::InvalidField("toBeDisplayed".into()))?;

            let mut displays: Vec<Cow<'a, Mutf8Str>> = vec![];

            for display in list {
                let str = (*display).to_owned();
                displays.push(Cow::Owned(str));
            }

            displays
        } else {
            vec![]
        };

        let is_filtering_craftable = get_bool(&nbt, "isFilteringCraftable");
        let is_gui_open = get_bool(&nbt, "isGuiOpen");
        let is_furnace_filtering_craftable = get_bool(&nbt, "isFurnaceFilteringCraftable");
        let is_furnace_gui_open = get_bool(&nbt, "isFurnaceGuiOpen");
        let is_blasting_furnace_filtering_craftable =
            get_bool(&nbt, "isBlastingFurnaceFilteringCraftable");
        let is_blast_furnace_gui_open = get_bool(&nbt, "isBlastingFurnaceGuiOpen");
        let is_smoker_filtering_craftable = get_bool(&nbt, "isSmokerFilteringCraftable");
        let is_smoker_gui_open = get_bool(&nbt, "isSmokerGuiOpen");

        Ok(RecipeBook {
            recipes,
            to_be_displayed,
            is_filtering_craftable,
            is_gui_open,
            is_furnace_filtering_craftable,
            is_furnace_gui_open,
            is_blasting_furnace_filtering_craftable,
            is_blast_furnace_gui_open,
            is_smoker_filtering_craftable,
            is_smoker_gui_open,
        })
    }
}
