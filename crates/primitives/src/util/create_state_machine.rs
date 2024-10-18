use std::fmt::Debug;

use leptos::prelude::*;

#[derive(Debug)]
pub struct InvalidState;
pub trait MachineState<T: Send + Sync, Event> {
  fn send(&self, event: Event) -> Result<T, InvalidState>;
}

pub fn create_state_machine<T: Send + Sync, Event: std::fmt::Debug + Copy>(
  initial_state: MaybeSignal<T>,
) -> (ReadSignal<T>, Callback<Event>)
where
  T: Clone + Debug + MachineState<T, Event>,
{
  let (state, set_state) = signal(initial_state.get_untracked());

  let send = move |event: Event| {
    set_state.update(|state| {
      if let Ok(result) = state.send(event) {
        *state = result;
      }
    });
  };

  (state, Callback::new(send))
}
