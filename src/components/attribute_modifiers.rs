use crate::{
    error::SculkParseError,
    traits::FromCompoundNbt,
    util::{get_bool, get_owned_mutf8str},
};
use simdnbt::Mutf8Str;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifiers<'a> {
    /// Show or hide attribute modifiers on this item's tooltip. Defaults to `true`.
    pub show_in_tooltip: bool,

    /// Contains attribute modifiers on this item which modify attributes of the wearer or holder (if the item is not in the hand or armor slots, it has no effect).
    pub modifiers: Vec<Modifier<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeModifier<'a> {
    ModifierList(Vec<Modifier<'a>>),

    Compound(AttributeModifiers<'a>),
}

/// A single attribute modifier.
#[derive(Debug, Clone, PartialEq)]
pub struct Modifier<'a> {
    /// The name of the attribute this modifier is to act upon.
    ///
    /// `type`
    pub r#type: Cow<'a, Mutf8Str>,

    /// Slot or slot type the item must be in for the modifier to take effect.
    pub slot: SlotType,

    /// The unique resource location to identify this modifier.
    pub id: Cow<'a, Mutf8Str>,

    /// Amount of change from the modifier.
    pub amount: f64,

    /// Modifier operation.
    pub operation: Operation,
}

/// Slot or slot type the item must be in for the modifier to take effect.
#[derive(Debug, Clone, PartialEq)]
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
pub enum Operation {
    /// `add_value`
    Add,
    /// `add_multiplied_base`
    MultiplyBase,
    /// `add_multiplied_total`
    MultiplyTotal,
}

impl<'a> FromCompoundNbt for AttributeModifier<'a> {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        if let Some(modifiers) = nbt.list("minecraft:attribute_modifiers") {
            let mods = modifiers
                .compounds()
                .ok_or(SculkParseError::InvalidField("modifiers".into()))?
                .into_iter()
                .map(|nbt| Modifier::from_compound_nbt(&nbt))
                .collect::<Result<Vec<Modifier>, SculkParseError>>()?;

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

impl<'a> FromCompoundNbt for AttributeModifiers<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let show_in_tooltip = get_bool(&nbt, "show_in_tooltip");

        let modifiers: Vec<Modifier<'a>> = match nbt.list("modifiers") {
            Some(modifiers) => modifiers
                .compounds()
                .ok_or(SculkParseError::InvalidField("modifiers".into()))?
                .into_iter()
                .map(|nbt| Modifier::from_compound_nbt(&nbt))
                .collect::<Result<Vec<Modifier>, SculkParseError>>()?,
            None => vec![],
        };

        Ok(AttributeModifiers {
            show_in_tooltip,
            modifiers,
        })
    }
}

impl<'a> FromCompoundNbt for Modifier<'a> {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let r#type = get_owned_mutf8str(nbt, "type")?;
        let slot = nbt
            .string("slot")
            .map(|s| SlotType::from_str(s.to_str().as_ref()))
            .ok_or(SculkParseError::MissingField("slot".into()))??;

        let id = get_owned_mutf8str(nbt, "id")?;

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
            "add_value" => Ok(Operation::Add),
            "add_multiplied_base" => Ok(Operation::MultiplyBase),
            "add_multiplied_total" => Ok(Operation::MultiplyTotal),
            _ => Err(SculkParseError::InvalidField(value.into())),
        }
    }
}
