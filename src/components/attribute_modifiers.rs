//! Attribute modifiers component.

use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_string, get_t_list},
};

/// A compound of attribute modifiers.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeModifiers {
    /// Show or hide attribute modifiers on this item's tooltip. Defaults to `true`.
    #[cfg_attr(feature = "serde", serde(default = "crate::util::default_true"))]
    pub show_in_tooltip: bool,

    /// Contains attribute modifiers on this item which modify attributes of the wearer or holder (if the item is not in the hand or armor slots, it has no effect).
    pub modifiers: Vec<Modifier>,
}

/// An attribute modifier component.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeModifier {
    /// A list of attribute modifiers.
    ModifierList(Vec<Modifier>),

    /// A compound of attribute modifiers.
    Compound(AttributeModifiers),
}

/// A single attribute modifier.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modifier {
    /// The name of the attribute this modifier is to act upon.
    ///
    /// `type`
    pub r#type: String,

    /// Slot or slot type the item must be in for the modifier to take effect.
    pub slot: SlotType,

    /// The unique resource location to identify this modifier.
    pub id: String,

    /// Amount of change from the modifier.
    pub amount: f64,

    /// Modifier operation.
    pub operation: Operation,
}

/// Slot or slot type the item must be in for the modifier to take effect.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum SlotType {
    /// `any`
    Any,
    /// `hand`
    Hand,
    /// `armor`
    Armor,
    /// `mainhand`
    MainHand,
    /// `offhand`
    OffHand,
    /// `head`
    Head,
    /// `chest`
    Chest,
    /// `legs`
    Legs,
    /// `feet`
    Feet,
    /// `body`
    Body,
}

/// Modifier operation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Operation {
    /// `add_value`
    AddValue,
    /// `add_multiplied_base`
    AddMultipliedBase,
    /// `add_multiplied_total`
    AddMultipliedTotal,
}

impl FromCompoundNbt for AttributeModifier {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(modifiers) = nbt.list("minecraft:attribute_modifiers") {
            let mods = get_t_list(
                &modifiers,
                "minecraft:attribute_modifiers",
                Modifier::from_compound_nbt,
            )?;

            Ok(AttributeModifier::ModifierList(mods))
        } else if let Some(compounded) = nbt.compound("minecraft:attribute_modifiers") {
            Ok(AttributeModifier::Compound(
                AttributeModifiers::from_compound_nbt(&compounded)?,
            ))
        } else {
            return Err(SculkParseError::MissingField(
                "minecraft:attribute_modifiers".into(),
            ));
        }
    }
}

impl FromCompoundNbt for AttributeModifiers {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let show_in_tooltip = get_bool(&nbt, "show_in_tooltip");

        let modifiers: Vec<Modifier> = match nbt.list("modifiers") {
            Some(modifiers) => get_t_list(&modifiers, "modifiers", Modifier::from_compound_nbt)?,
            None => vec![],
        };

        Ok(AttributeModifiers {
            show_in_tooltip,
            modifiers,
        })
    }
}

impl FromCompoundNbt for Modifier {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let r#type = get_owned_string(nbt, "type")?;
        let slot = nbt
            .string("slot")
            .map(|s| SlotType::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("slot".into()))??;

        let id = get_owned_string(nbt, "id")?;

        let amount = nbt
            .double("amount")
            .ok_or(SculkParseError::MissingField("amount".into()))?;

        let operation = nbt
            .string("operation")
            .map(|s| Operation::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("operation".into()))??;

        Ok(Modifier {
            r#type,
            slot,
            id,
            amount,
            operation,
        })
    }
}

impl SlotType {
    fn from_str(value: &str) -> Result<Self, SculkParseError> {
        match value {
            "any" => Ok(SlotType::Any),
            "hand" => Ok(SlotType::Hand),
            "armor" => Ok(SlotType::Armor),
            "mainhand" => Ok(SlotType::MainHand),
            "offhand" => Ok(SlotType::OffHand),
            "head" => Ok(SlotType::Head),
            "chest" => Ok(SlotType::Chest),
            "legs" => Ok(SlotType::Legs),
            "feet" => Ok(SlotType::Feet),
            "body" => Ok(SlotType::Body),
            _ => Err(SculkParseError::InvalidField(value.into())),
        }
    }
}

impl Operation {
    fn from_str(value: &str) -> Result<Self, SculkParseError> {
        match value {
            "add_value" => Ok(Operation::AddValue),
            "add_multiplied_base" => Ok(Operation::AddMultipliedBase),
            "add_multiplied_total" => Ok(Operation::AddMultipliedTotal),
            _ => Err(SculkParseError::InvalidField(value.into())),
        }
    }
}
