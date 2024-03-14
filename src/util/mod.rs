pub(crate) mod create_controllable_signal;
pub(crate) mod create_id;
pub(crate) mod create_previous;
pub(crate) mod create_state_machine;

#[derive(Default, Clone, PartialEq)]
pub enum Direction {
  #[default]
  LeftToRight,
  RightToLeft,
}

#[derive(Default, Clone, PartialEq)]
pub enum Orientation {
  #[default]
  Horizontal,
  Vertical,
}
