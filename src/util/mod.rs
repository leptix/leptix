pub(crate) mod create_controllable_signal;
pub(crate) mod create_id;
pub(crate) mod create_previous;
pub(crate) mod create_state_machine;

#[derive(Clone, PartialEq)]
pub enum Direction {
  LeftToRight,
  RightToLeft,
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
  Horizontal,
  Vertical,
}
