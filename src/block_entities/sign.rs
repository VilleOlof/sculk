use crate::{color::Color, error::SculkParseError, traits::FromCompoundNbt, util::get_bool};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sign {
    /// true if the text is locked with honeycomb.
    pub is_waxed: bool,

    /// Discribes front text.
    pub front_text: SignText,

    /// Discribes back text
    pub back_text: SignText,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignText {
    /// true if the sign has been dyed with a glow ink sac.
    pub has_glowing_text: bool,

    /// The color that has been used to dye the sign. The default value is black.
    pub color: Color,

    /// Only in Realms. The lines of text shown to players with the profanity filter turned on instead of the regular lines. This tag is automatically set to "" for lines containing blocked words and to the line's normal contents for the other lines when a player with the profanity filter turned off edits the sign, so players with the filter on cannot see the blocked words. If a player with the filter on tries to use blocked words in one or more lines, the line(s) in  messages containing blocked words are set to "", which makes them render completely blank, and this tag is also given the same contents. If multiple lines have been edited before the sign editing GUI is closed, only the lines containing blocked words are blanked.
    pub filtered_messages: Option<Vec<String>>,

    /// A list of text for each line.
    pub messages: Vec<String>,
}

impl FromCompoundNbt for Sign {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let is_waxed = get_bool(&nbt, "is_waxed");

        let front_text = nbt
            .compound("front_text")
            .map(|nbt| SignText::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("front_text".into()))??;

        let back_text = nbt
            .compound("back_text")
            .map(|nbt| SignText::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("back_text".into()))??;

        Ok(Sign {
            is_waxed,
            front_text,
            back_text,
        })
    }
}

impl FromCompoundNbt for SignText {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let has_glowing_text = get_bool(&nbt, "has_glowing_text");

        let color = nbt
            .string("color")
            .map(|s| {
                Color::from_str(s.to_str().as_ref())
                    .ok_or(SculkParseError::InvalidField("color".into()))
            })
            .ok_or(SculkParseError::MissingField("color".into()))??;

        let filtered_messages = if let Some(list) = nbt.list("filtered_messages") {
            let mut filtered_messages: Vec<String> = vec![];

            for message in list.strings().into_iter() {
                let str = (*message.first().unwrap()).to_string();
                filtered_messages.push(str);
            }

            Some(filtered_messages)
        } else {
            None
        };

        let messages_list = nbt
            .list("messages")
            .ok_or(SculkParseError::MissingField("messages".into()))?;
        let mut messages: Vec<String> = vec![];

        for message in messages_list.strings().into_iter() {
            let str = (*message.first().unwrap()).to_string();
            messages.push(str);
        }

        Ok(SignText {
            has_glowing_text,
            color,
            filtered_messages,
            messages,
        })
    }
}
