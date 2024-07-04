use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BlockState<'a>(HashMap<Cow<'a, str>, Cow<'a, str>>);
