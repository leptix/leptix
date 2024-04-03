use leptos::*;

#[derive(Debug)]
pub(crate) struct InvalidState;
pub(crate) trait MachineState<T, Event> {
  fn send(&self, event: Event) -> Result<T, InvalidState>;
}

pub(crate) fn create_state_machine<T: std::fmt::Debug, Event: std::fmt::Debug + Copy>(
  initial_state: MaybeSignal<T>,
) -> (ReadSignal<T>, Callback<Event>)
where
  T: MachineState<T, Event> + Clone,
{
  let (state, set_state) = create_signal(initial_state.get());

  let send = move |event: Event| {
    logging::log!("sending ev in state machine");

    set_state.update(|state| {
      logging::log!("updating state");
      if let Ok(result) = state.send(event) {
        logging::log!("valid state, setting");
        *state = result;
        logging::log!("state set");
      }

      match state.send(event) {
        Ok(result) => {
          *state = result;
        }
        Err(_) => {
          logging::log!("invalid state: {state:?} {event:?}");
        }
      }

      logging::log!("done updating");
    });

    logging::log!("done sending ev in state machine");
  };

  (state, Callback::new(send))
}
