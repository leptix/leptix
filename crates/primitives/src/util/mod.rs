pub(crate) mod create_controllable_signal;
pub(crate) mod create_id;
pub(crate) mod create_previous;
pub(crate) mod create_state_machine;

pub(crate) fn linear_scale(
  (input_start, input_end): (f64, f64),
  (output_start, output_end): (f64, f64),
) -> impl Fn(f64) -> f64 {
  move |value| {
    if input_start == input_end || output_start == output_end {
      return output_start;
    }

    let ratio = (output_end - output_start) / (input_end - input_start);
    output_start + ratio * (value - input_start)
  }
}

// pub(crate) type AttributePair = (&'static str, leptos::attr::Attribute);
// pub(crate) type Attributes = Vec<AttributePair>;
