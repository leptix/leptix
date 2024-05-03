use std::fmt::Debug;

use leptos::*;

#[derive(Debug)]
pub struct InvalidState;
pub trait MachineState<T, Event> {
  fn send(&self, event: Event) -> Result<T, InvalidState>;
}

pub fn create_state_machine<T, Event: std::fmt::Debug + Copy>(
  initial_state: MaybeSignal<T>,
) -> (ReadSignal<T>, Callback<Event>)
where
  T: Clone + Debug + MachineState<T, Event>,
{
  let (state, set_state) = create_signal(initial_state.get());

  let send = move |event: Event| {
    set_state.update(|state| {
      // if let Ok(result) = state.send(event) {
      //   *state = result;
      // }

      match state.send(event) {
        Ok(result) => {
          *state = result;
        }
        Err(_) => {
          logging::log!("invalid state: {state:?} {event:?}");
        }
      }
    });
  };

  (state, Callback::new(send))
}
