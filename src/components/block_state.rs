use std::{borrow::Cow, collections::HashMap};

use simdnbt::Mutf8Str;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockState<'a>(HashMap<Cow<'a, str>, Cow<'a, Mutf8Str>>);
