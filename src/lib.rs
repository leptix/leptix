#![feature(trait_alias)]
#![feature(iter_partition_in_place)]

pub mod components;
pub mod util;

pub type AttributePair = (&'static str, leptos::Attribute);
pub type Attributes = Vec<AttributePair>;
