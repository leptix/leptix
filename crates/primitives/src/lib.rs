pub mod components;

pub(crate) mod util;

pub use components::*;

pub(crate) type AttributePair = (&'static str, leptos::Attribute);
pub(crate) type Attributes = Vec<AttributePair>;
