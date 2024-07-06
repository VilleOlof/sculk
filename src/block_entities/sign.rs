use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Sign<'a> {
    /// true if the text is locked with honeycomb.
    pub is_waxed: bool,

    /// Discribes front text.
    #[serde(borrow)]
    pub front_text: SignText<'a>,

    /// Discribes back text
    #[serde(borrow)]
    pub back_text: SignText<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SignText<'a> {
    /// true if the sign has been dyed with a glow ink sac.
    pub has_glowing_text: bool,

    /// The color that has been used to dye the sign. The default value is black.
    pub color: Color,

    /// Only in Realms. The lines of text shown to players with the profanity filter turned on instead of the regular lines. This tag is automatically set to "" for lines containing blocked words and to the line's normal contents for the other lines when a player with the profanity filter turned off edits the sign, so players with the filter on cannot see the blocked words. If a player with the filter on tries to use blocked words in one or more lines, the line(s) in  messages containing blocked words are set to "", which makes them render completely blank, and this tag is also given the same contents. If multiple lines have been edited before the sign editing GUI is closed, only the lines containing blocked words are blanked.
    pub filtered_messages: Vec<Cow<'a, str>>,

    /// A list of text for each line.
    pub messages: Vec<Cow<'a, str>>,
}
