use leptos::{Attribute, IntoAttribute, Oco};
use strum::{Display, EnumString, IntoStaticStr};

pub mod accordion;
pub mod aspect_ratio;
pub mod avatar;
pub mod checkbox;
pub mod collapsible;
pub(crate) mod dismissable_layer;
pub mod label;
pub(crate) mod popper;
pub mod primitive;
pub mod progress;
pub(crate) mod radio;
pub mod radio_group;
pub mod scroll_area;
pub mod separator;
pub mod slider;
pub mod slot;
pub mod switch;
pub mod tabs;
pub mod toggle;
pub mod toggle_group;
pub mod toolbar;

pub(crate) mod collection;
pub(crate) mod presence;
pub(crate) mod roving_focus;

#[derive(Default, Clone, PartialEq, Copy, Display, IntoStaticStr)]
pub enum Direction {
  #[default]
  #[strum(serialize = "ltr")]
  LeftToRight,
  #[strum(serialize = "rtl")]
  RightToLeft,
}

#[derive(Default, Clone, PartialEq, Copy, Display, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Orientation {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Clone, PartialEq, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum Side {
  Top,
  Right,
  Bottom,
  Left,
}

impl IntoAttribute for Direction {
  fn into_attribute(self) -> Attribute {
    Attribute::String(Oco::Borrowed(self.into()))
  }

  fn into_attribute_boxed(self: Box<Self>) -> Attribute {
    Attribute::String(Oco::Borrowed(self.as_ref().into()))
  }
}

impl IntoAttribute for Orientation {
  fn into_attribute(self) -> Attribute {
    Attribute::String(Oco::Borrowed(self.into()))
  }

  fn into_attribute_boxed(self: Box<Self>) -> Attribute {
    Attribute::String(Oco::Borrowed(self.as_ref().into()))
  }
}

impl IntoAttribute for Side {
  fn into_attribute(self) -> Attribute {
    Attribute::String(Oco::Borrowed(self.into()))
  }

  fn into_attribute_boxed(self: Box<Self>) -> Attribute {
    Attribute::String(Oco::Borrowed(self.as_ref().into()))
  }
}
